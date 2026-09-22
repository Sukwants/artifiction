import {mkdir, writeFile} from "node:fs/promises"
import {resolve} from "node:path"
import type {ConsoleMessage, HTTPRequest} from "puppeteer-core"
import {launchMonaApiBrowser} from "./browser"
import {
    MonaApiRemoteError,
    type MonaApiClient,
    type MonaApiErrorInfo,
    type MonaApiEvent,
    type MonaApiInvocationResult,
    type MonaApiPage,
    type MonaApiTestCallback,
    type MonaApiTestDiagnostics,
    type MonaApiTestOptions,
    type MonaApiTestSession,
    type MonaNavigationResult
} from "./types"

interface RemoteCallInput {
    path: string
    args: unknown[]
}

function serializeError(error: unknown): MonaApiErrorInfo {
    if (error && typeof error === "object") {
        const candidate = error as Record<string, unknown>
        return {
            name: typeof candidate.name === "string" ? candidate.name : "Error",
            code: typeof candidate.code === "string" ? candidate.code : undefined,
            message: typeof candidate.message === "string" ? candidate.message : String(error),
            details: candidate.details
        }
    }
    return {
        name: "Error",
        message: String(error)
    }
}

async function evaluateMonaCall<T>(input: RemoteCallInput): Promise<MonaApiInvocationResult<T>> {
    try {
        let owner: any = (globalThis as any).monaApi
        let target: any = owner
        for (const part of input.path.split(".")) {
            owner = target
            target = target?.[part]
        }
        if (typeof target !== "function") {
            throw new Error(`Mona API method does not exist: ${input.path}`)
        }
        const value = await target.apply(owner, input.args)
        return {ok: true, value}
    } catch (error) {
        return {ok: false, error: serializeError(error)}
    }
}

type EventCallResult<T> = MonaApiInvocationResult<T> & {
    events: MonaApiEvent[]
}

async function evaluateMonaCallWithEvents<T>(input: RemoteCallInput): Promise<EventCallResult<T>> {
    const events: MonaApiEvent[] = []
    const api: any = (globalThis as any).monaApi
    const unsubscribe = api.events.subscribe((event: MonaApiEvent) => events.push(event))
    try {
        let owner: any = api
        let target: any = owner
        for (const part of input.path.split(".")) {
            owner = target
            target = target?.[part]
        }
        if (typeof target !== "function") {
            throw new Error(`Mona API method does not exist: ${input.path}`)
        }
        const value = await target.apply(owner, input.args)
        return {ok: true, value, events}
    } catch (error) {
        return {ok: false, error: serializeError(error), events}
    } finally {
        unsubscribe()
    }
}

function normalizeOptions(options: MonaApiTestOptions): Required<Pick<
    MonaApiTestOptions,
    "baseUrl" | "defaultTimeout" | "navigationTimeout"
>> & MonaApiTestOptions {
    return {
        ...options,
        baseUrl: (options.baseUrl ?? process.env.MONA_E2E_BASE_URL ?? "http://127.0.0.1:8080").replace(/\/+$/, ""),
        defaultTimeout: options.defaultTimeout ?? Number(process.env.MONA_E2E_TIMEOUT ?? 30000),
        navigationTimeout: options.navigationTimeout ?? Number(process.env.MONA_E2E_NAVIGATION_TIMEOUT ?? 30000),
        captureOnFailure: options.captureOnFailure ?? true
    }
}

function createApiProxy(
    version: string,
    invoke: <T = unknown>(path: string, ...args: unknown[]) => Promise<T>
): MonaApiClient {
    function createNamespace(path: string[]): any {
        const callable = () => undefined
        return new Proxy(callable, {
            get(_target, property) {
                if (typeof property !== "string") return undefined
                if (property === "then") return undefined
                if (path.length === 0 && property === "version") return version
                if (path.length === 0 && (property === "call" || property === "invoke")) return invoke
                return createNamespace([...path, property])
            },
            apply(_target, _thisArg, args) {
                if (path.length === 0) throw new Error("Mona API root is not callable")
                return invoke(path.join("."), ...args)
            }
        })
    }

    return createNamespace([]) as MonaApiClient
}

function addConsoleDiagnostic(diagnostics: MonaApiTestDiagnostics, message: ConsoleMessage) {
    const location = message.location()
    diagnostics.console.push({
        type: message.type(),
        text: message.text(),
        location: location.url
            ? `${location.url}:${location.lineNumber ?? 0}:${location.columnNumber ?? 0}`
            : undefined
    })
}

function addRequestDiagnostic(diagnostics: MonaApiTestDiagnostics, request: HTTPRequest) {
    const failure = request.failure()
    diagnostics.failedRequests.push({
        url: request.url(),
        method: request.method(),
        failure: failure?.errorText
    })
}

function safeName(value: string): string {
    return value.replace(/[^a-zA-Z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") || "project-e2e"
}

async function saveFailureArtifacts(
    session: Pick<MonaApiTestSession, "page" | "diagnostics" | "options">,
    error: unknown
) {
    if (session.options.captureOnFailure === false) return

    const directory = resolve(
        session.options.artifactsDir ?? process.env.MONA_E2E_ARTIFACTS_DIR ?? "test-results/project"
    )
    const prefix = `${safeName(session.options.name ?? "project-e2e")}-${Date.now()}`
    try {
        await mkdir(directory, {recursive: true})
        await session.page.screenshot({path: resolve(directory, `${prefix}.png`), fullPage: true})
        await writeFile(resolve(directory, `${prefix}.html`), await session.page.content(), "utf8")
        await writeFile(
            resolve(directory, `${prefix}.json`),
            JSON.stringify({
                error: {
                    message: error instanceof Error ? error.message : String(error),
                    stack: error instanceof Error ? error.stack : undefined
                },
                diagnostics: session.diagnostics
            }, null, 2),
            "utf8"
        )
    } catch {
        // Failure diagnostics must not mask the original test failure.
    }
}

export async function createMonaApiTestSession(
    inputOptions: MonaApiTestOptions = {}
): Promise<MonaApiTestSession> {
    const options = normalizeOptions(inputOptions)
    const browser = await launchMonaApiBrowser(options)
    const context = await browser.createBrowserContext()
    const page = await context.newPage()
    const diagnostics: MonaApiTestDiagnostics = {
        console: [],
        pageErrors: [],
        pageCrashes: [],
        failedRequests: []
    }

    page.on("console", message => addConsoleDiagnostic(diagnostics, message))
    page.on("pageerror", error => diagnostics.pageErrors.push(
        error instanceof Error ? (error.stack ?? error.message) : String(error)
    ))
    page.on("error", error => diagnostics.pageCrashes.push(
        error instanceof Error ? (error.stack ?? error.message) : String(error)
    ))
    page.on("requestfailed", request => addRequestDiagnostic(diagnostics, request))

    const invokeResult = async <T = unknown>(path: string, ...args: unknown[]) => {
        return page.evaluate(evaluateMonaCall<T>, {path, args})
    }

    const invoke = async <T = unknown>(path: string, ...args: unknown[]): Promise<T> => {
        const result = await invokeResult<T>(path, ...args)
        if (!result.ok) throw new MonaApiRemoteError(result.error)
        return result.value
    }

    let closed = false
    try {
        page.setDefaultTimeout(options.defaultTimeout)
        page.setDefaultNavigationTimeout(options.navigationTimeout)
        if (options.viewport) await page.setViewport(options.viewport)
        await page.goto(options.baseUrl, {
            waitUntil: "domcontentloaded",
            timeout: options.navigationTimeout
        })
        await page.waitForFunction(
            () => typeof (window as any).monaApi !== "undefined",
            {timeout: options.defaultTimeout}
        )
        const version = await page.evaluate(() => (window as any).monaApi.version as string)
        const api = createApiProxy(version, invoke)
        await api.ready()

        const session: MonaApiTestSession = {
            browser,
            context,
            page,
            api,
            diagnostics,
            options: {
                ...options,
                baseUrl: options.baseUrl,
                defaultTimeout: options.defaultTimeout,
                navigationTimeout: options.navigationTimeout
            },
            invokeResult,
            invoke,
            call: invoke,
            async expectError(path, expectedCode, ...args) {
                const result = await invokeResult(path, ...args)
                if (result.ok) {
                    throw new Error(`Expected ${path} to fail, but it resolved successfully`)
                }
                if (expectedCode && result.error.code !== expectedCode) {
                    throw new Error(
                        `Expected ${path} to fail with ${expectedCode}, got ${result.error.code ?? "unknown"}`
                    )
                }
                return result.error
            },
            async captureEvents<T = unknown>(path: string, ...args: unknown[]) {
                const result = await page.evaluate(evaluateMonaCallWithEvents<T>, {path, args})
                if (!result.ok) throw new MonaApiRemoteError(result.error)
                return {value: result.value, events: result.events}
            },
            navigate(pageName: MonaApiPage) {
                return invoke<MonaNavigationResult>("navigation.enter", pageName)
            },
            async reload() {
                await page.reload({
                    waitUntil: "domcontentloaded",
                    timeout: options.navigationTimeout
                })
                await page.waitForFunction(
                    () => typeof (window as any).monaApi !== "undefined",
                    {timeout: options.defaultTimeout}
                )
                await invoke("ready")
            },
            saveFailureArtifacts(error) {
                return saveFailureArtifacts(session, error)
            },
            async close() {
                if (closed) return
                closed = true
                await context.close()
                await browser.close()
            }
        }

        return session
    } catch (error) {
        if (!closed) {
            closed = true
            await context.close().catch(() => undefined)
            await browser.close().catch(() => undefined)
        }
        throw error
    }
}

export async function withMonaApiTest<T>(
    callback: MonaApiTestCallback<T>,
    options: MonaApiTestOptions = {}
): Promise<T> {
    const session = await createMonaApiTestSession(options)
    try {
        return await callback(session)
    } catch (error) {
        await session.saveFailureArtifacts(error)
        throw error
    } finally {
        await session.close()
    }
}
