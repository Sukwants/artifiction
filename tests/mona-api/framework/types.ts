import type {Browser, BrowserContext, LaunchOptions, Page} from "puppeteer-core"

export type MonaApiPage =
    | "accounts"
    | "artifacts"
    | "artifactLoadouts"
    | "presets"
    | "calculator"

export interface MonaApiCapabilities {
    accounts: boolean
    artifacts: boolean
    artifactLoadouts: boolean
    presets: boolean
    calculator: boolean
    navigation: boolean
    damageAnalysis: string
    [key: string]: unknown
}

export interface MonaNavigationResult {
    page: MonaApiPage
    path: string
    routeName: string | null
}

export interface MonaApiEvent {
    type: string
    revision: number
    source: "api" | "page"
    data?: unknown
}

export interface MonaApiErrorInfo {
    name: string
    code?: string
    message: string
    details?: unknown
}

export interface MonaApiInvocationSuccess<T> {
    ok: true
    value: T
}

export interface MonaApiInvocationFailure {
    ok: false
    error: MonaApiErrorInfo
}

export type MonaApiInvocationResult<T> =
    | MonaApiInvocationSuccess<T>
    | MonaApiInvocationFailure

export class MonaApiRemoteError extends Error {
    readonly code?: string
    readonly details?: unknown
    readonly remoteName: string

    constructor(error: MonaApiErrorInfo) {
        super(error.message)
        this.name = "MonaApiRemoteError"
        this.remoteName = error.name
        this.code = error.code
        this.details = error.details
    }
}

export interface MonaApiNamespace {
    [key: string]: any
}

export interface MonaApiClient {
    readonly version: string
    readonly accounts: MonaApiNamespace
    readonly artifacts: MonaApiNamespace
    readonly artifactLoadouts: MonaApiNamespace
    readonly presets: MonaApiNamespace
    readonly navigation: MonaApiNamespace
    readonly calculator: MonaApiNamespace
    readonly meta: MonaApiNamespace
    readonly events: MonaApiNamespace
    ready(): Promise<void>
    capabilities(): Promise<MonaApiCapabilities>
    call<T = unknown>(path: string, ...args: unknown[]): Promise<T>
    invoke<T = unknown>(path: string, ...args: unknown[]): Promise<T>
}

export interface MonaApiTestOptions {
    /** Human-readable suite name used in failure artifacts. */
    name?: string
    /** URL of the running Vue application. Defaults to MONA_E2E_BASE_URL or localhost:8080. */
    baseUrl?: string
    /** Path to Chrome/Edge. Defaults to MONA_E2E_BROWSER_PATH or auto-detection. */
    executablePath?: string
    /** Project runner debug mode: keep the browser visible and pause after each suite. */
    debug?: boolean
    headless?: boolean | "shell"
    slowMo?: number
    args?: string[]
    viewport?: {
        width: number
        height: number
    }
    defaultTimeout?: number
    navigationTimeout?: number
    artifactsDir?: string
    captureOnFailure?: boolean
    launchOptions?: Omit<LaunchOptions, "args" | "executablePath" | "headless" | "slowMo">
}

export interface MonaApiTestDiagnostics {
    console: Array<{
        type: string
        text: string
        location?: string
    }>
    pageErrors: string[]
    pageCrashes: string[]
    failedRequests: Array<{
        url: string
        method: string
        failure?: string
    }>
}

export interface MonaApiTestSession {
    readonly browser: Browser
    readonly context: BrowserContext
    readonly page: Page
    readonly api: MonaApiClient
    readonly diagnostics: MonaApiTestDiagnostics
    readonly options: Required<Pick<MonaApiTestOptions, "baseUrl" | "defaultTimeout" | "navigationTimeout">>
        & MonaApiTestOptions

    invokeResult<T = unknown>(path: string, ...args: unknown[]): Promise<MonaApiInvocationResult<T>>
    invoke<T = unknown>(path: string, ...args: unknown[]): Promise<T>
    call<T = unknown>(path: string, ...args: unknown[]): Promise<T>
    expectError(path: string, expectedCode?: string, ...args: unknown[]): Promise<MonaApiErrorInfo>
    captureEvents<T = unknown>(path: string, ...args: unknown[]): Promise<{
        value: T
        events: MonaApiEvent[]
    }>
    navigate(page: MonaApiPage): Promise<MonaNavigationResult>
    reload(): Promise<void>
    saveFailureArtifacts(error: unknown): Promise<void>
    close(): Promise<void>
}

export type MonaApiTestCallback<T = unknown> = (session: MonaApiTestSession) => Promise<T>
