import {existsSync} from "node:fs"
import {join} from "node:path"
import puppeteer, {type Browser} from "puppeteer-core"
import type {MonaApiTestOptions} from "./types"

function existingPath(value: string | undefined): string | undefined {
    if (!value) return undefined
    return existsSync(value) ? value : undefined
}

function windowsBrowserCandidates(): string[] {
    const roots = [
        process.env.PROGRAMFILES,
        process.env["PROGRAMFILES(X86)"],
        process.env.LOCALAPPDATA
    ].filter((value): value is string => Boolean(value))

    return roots.flatMap(root => [
        join(root, "Google", "Chrome", "Application", "chrome.exe"),
        join(root, "Microsoft", "Edge", "Application", "msedge.exe")
    ])
}

function platformBrowserCandidates(): string[] {
    if (process.platform === "win32") return windowsBrowserCandidates()
    if (process.platform === "darwin") {
        return [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"
        ]
    }
    return [
        "/usr/bin/google-chrome",
        "/usr/bin/google-chrome-stable",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "/snap/bin/chromium"
    ]
}

export function resolveBrowserExecutablePath(explicitPath?: string): string {
    const configured = explicitPath ?? process.env.MONA_E2E_BROWSER_PATH
    if (configured) {
        const path = existingPath(configured)
        if (!path) {
            throw new Error(`Configured browser executable does not exist: ${configured}`)
        }
        return path
    }

    const path = [
        ...platformBrowserCandidates()
    ].map(existingPath).find((value): value is string => Boolean(value))

    if (!path) {
        throw new Error(
            "Cannot find Chrome or Edge. Set MONA_E2E_BROWSER_PATH to a browser executable when using puppeteer-core."
        )
    }
    return path
}

function resolveHeadless(
    value: MonaApiTestOptions["headless"],
    debug = false
): boolean | "shell" {
    if (debug) return false
    if (value !== undefined) return value
    const configured = process.env.MONA_E2E_HEADLESS?.trim().toLowerCase()
    if (configured === "false" || configured === "0" || configured === "no") return false
    if (configured === "shell") return "shell"
    return true
}

function resolveArgs(args: string[] | undefined): string[] {
    const result = [...(args ?? [])]
    const noSandbox = process.env.MONA_E2E_NO_SANDBOX === "true"
    if (noSandbox) {
        result.push("--no-sandbox", "--disable-setuid-sandbox")
    }
    return [...new Set(result)]
}

export async function launchMonaApiBrowser(options: MonaApiTestOptions = {}): Promise<Browser> {
    const executablePath = resolveBrowserExecutablePath(options.executablePath)
    return puppeteer.launch({
        ...options.launchOptions,
        executablePath,
        headless: resolveHeadless(options.headless, options.debug),
        slowMo: options.slowMo,
        args: resolveArgs(options.args)
    })
}
