import {resolve} from "node:path"
import {fileURLToPath} from "node:url"
import {
    discoverProjectTestSuites,
    runProjectTestSuites,
    type MonaApiTestOptions,
    type ProjectTestRunReport
} from "./framework"

function optionValue(args: readonly string[], name: string): string | undefined {
    const index = args.indexOf(name)
    return index >= 0 ? args[index + 1] : undefined
}

function suiteIdsFromArgs(args: readonly string[]): string[] | undefined {
    const value = optionValue(args, "--suite")
    if (!value) return undefined
    return value.split(",").map(item => item.trim()).filter(Boolean)
}

function sessionOptionsFromArgs(args: readonly string[]): MonaApiTestOptions | undefined {
    if (args.includes("--headed")) {
        throw new Error("--headed 已替换为 --debug，请使用 --debug")
    }
    const debug = args.includes("--debug")
    const headless = args.includes("--headless")
    if (debug && headless) {
        throw new Error("--debug 和 --headless 不能同时使用")
    }

    const executablePath = optionValue(args, "--browser-path") ?? optionValue(args, "--browser")
    const options: MonaApiTestOptions = {}
    if (debug) {
        options.debug = true
        options.headless = false
    }
    if (headless) options.headless = true
    if (executablePath) options.executablePath = executablePath
    return Object.keys(options).length > 0 ? options : undefined
}

async function printDiscovery(args: readonly string[]): Promise<number> {
    const report = await discoverProjectTestSuites({
        rootDir: optionValue(args, "--root"),
        include: suiteIdsFromArgs(args)
    })
    for (const suite of report.suites) {
        console.log(`[discovered] ${suite.suiteId}: ${suite.suite.cases.length} case(s)`)
    }
    for (const issue of report.issues) {
        console.log(`[${issue.type}] ${issue.suiteId}: ${issue.message}`)
    }
    return report.issues.length > 0 ? 1 : 0
}

function printReport(report: ProjectTestRunReport): void {
    for (const result of report.results) {
        const caseName = result.caseId ? `/${result.caseId}` : ""
        const message = result.message ? ` - ${result.message}` : ""
        console.log(`[${result.severity}] ${result.suiteId}${caseName}: ${result.type}${message}`)
    }
    console.log(
        `Summary: ${report.summary.passed}/${report.summary.total} passed, `
        + `${report.summary.failed} failed`
    )
}

export async function main(args: readonly string[] = process.argv.slice(2)): Promise<number> {
    const rootDir = optionValue(args, "--root")
    if (args.includes("--list")) {
        return printDiscovery(args)
    }

    const report = await runProjectTestSuites({
        rootDir,
        suiteIds: suiteIdsFromArgs(args),
        sessionOptions: sessionOptionsFromArgs(args),
        continueOnError: !args.includes("--fail-fast")
    })
    printReport(report)
    return report.summary.failed === 0 ? 0 : 1
}

const currentFile = resolve(fileURLToPath(import.meta.url))
const invokedFile = process.argv[1] ? resolve(process.argv[1]) : ""
if (currentFile === invokedFile) {
    main().then(code => {
        process.exitCode = code
    }).catch(error => {
        console.error(error)
        process.exitCode = 1
    })
}
