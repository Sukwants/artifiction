import type {
    MonaApiTestOptions,
    MonaApiTestSession
} from "../mona-api/framework"

export type MonaApiTestCaseCategory = "general" | "calculation"

export type MonaApiTestResultType =
    | "passed"
    | "calculation-error"
    | "fixture-missing"
    | "fixture-invalid"
    | "import-error"
    | "loadout-error"
    | "damage-out-of-tolerance"
    | "page-crashed"
    | "test-root-missing"
    | "test-file-missing"
    | "suite-import-error"
    | "suite-invalid"
    | "timeout"
    | "assertion-error"
    | "test-failed"
    | "framework-error"

export type MonaApiTestFailureType = Exclude<MonaApiTestResultType, "passed">

export type MonaApiTestResultSeverity = "success" | "warning" | "error" | "fatal"

export interface MonaApiSerializedError {
    name: string
    message: string
    code?: string
    details?: unknown
    stack?: string
}

export type MonaApiTestCaseRunner = (
    session: MonaApiTestSession
) => unknown | Promise<unknown>

export interface MonaApiTestCaseDefinition {
    id: string
    name?: string
    category?: MonaApiTestCaseCategory
    run: MonaApiTestCaseRunner
}

export class MonaApiTestFailure extends Error {
    readonly resultType: MonaApiTestFailureType
    readonly details?: unknown

    constructor(resultType: MonaApiTestFailureType, message: string, details?: unknown) {
        super(message)
        this.name = "MonaApiTestFailure"
        this.resultType = resultType
        this.details = details
    }
}

export interface MonaApiTestSuiteDefinition {
    id: string
    name?: string
    description?: string
    options?: MonaApiTestOptions
    cases: readonly MonaApiTestCaseDefinition[]
}

export interface MonaApiTestSuite extends Omit<MonaApiTestSuiteDefinition, "name" | "cases"> {
    name: string
    cases: readonly MonaApiTestCaseDefinition[]
}

export class MonaApiSuiteDefinitionError extends Error {
    readonly issues: readonly string[]

    constructor(issues: readonly string[]) {
        super(`Invalid project test suite: ${issues.join("; ")}`)
        this.name = "MonaApiSuiteDefinitionError"
        this.issues = issues
    }
}

export function validateMonaApiTestSuite(value: unknown): string[] {
    if (!value || typeof value !== "object") return ["suite must be an object"]

    const suite = value as Partial<MonaApiTestSuiteDefinition>
    const issues: string[] = []
    if (typeof suite.id !== "string" || suite.id.trim() === "") {
        issues.push("id must be a non-empty string")
    }
    if (suite.name !== undefined && typeof suite.name !== "string") {
        issues.push("name must be a string when provided")
    }
    if (!Array.isArray(suite.cases) || suite.cases.length === 0) {
        issues.push("cases must be a non-empty array")
        return issues
    }

    const ids = new Set<string>()
    for (const [index, testCase] of suite.cases.entries()) {
        if (!testCase || typeof testCase !== "object") {
            issues.push(`cases[${index}] must be an object`)
            continue
        }
        const candidate = testCase as Partial<MonaApiTestCaseDefinition>
        if (typeof candidate.id !== "string" || candidate.id.trim() === "") {
            issues.push(`cases[${index}].id must be a non-empty string`)
        } else if (ids.has(candidate.id)) {
            issues.push(`duplicate case id: ${candidate.id}`)
        } else {
            ids.add(candidate.id)
        }
        if (candidate.name !== undefined && typeof candidate.name !== "string") {
            issues.push(`cases[${index}].name must be a string when provided`)
        }
        if (candidate.category !== undefined && candidate.category !== "general" && candidate.category !== "calculation") {
            issues.push(`cases[${index}].category must be general or calculation`)
        }
        if (typeof candidate.run !== "function") {
            issues.push(`cases[${index}].run must be a function`)
        }
    }
    return issues
}

export function defineMonaApiTestSuite(input: MonaApiTestSuiteDefinition): MonaApiTestSuite {
    const issues = validateMonaApiTestSuite(input)
    if (issues.length > 0) throw new MonaApiSuiteDefinitionError(issues)

    return Object.freeze({
        ...input,
        id: input.id.trim(),
        name: input.name?.trim() || input.id.trim(),
        cases: Object.freeze(input.cases.map(testCase => Object.freeze({
            ...testCase,
            id: testCase.id.trim(),
            name: testCase.name?.trim() || testCase.id.trim()
        })))
    })
}

export interface MonaApiDiscoveredSuite {
    suite: MonaApiTestSuite
    suiteId: string
    directory: string
    entryFile: string
}

export type MonaApiDiscoveryIssueType =
    | "test-root-missing"
    | "test-file-missing"
    | "suite-import-error"
    | "suite-invalid"

export interface MonaApiDiscoveryIssue {
    type: MonaApiDiscoveryIssueType
    suiteId: string
    directory: string
    entryFile: string
    message: string
    error?: MonaApiSerializedError
}

export interface MonaApiSuiteDiscoveryOptions {
    /** Defaults to <project>/tests/suites. */
    rootDir?: string
    /** Defaults to suite.ts. */
    entryFile?: string
    include?: readonly string[]
    exclude?: readonly string[]
}

export interface MonaApiSuiteDiscoveryReport {
    rootDir: string
    suites: MonaApiDiscoveredSuite[]
    issues: MonaApiDiscoveryIssue[]
}

export interface MonaApiTestResult {
    suiteId: string
    suiteName?: string
    caseId?: string
    caseName?: string
    type: MonaApiTestResultType
    severity: MonaApiTestResultSeverity
    ok: boolean
    durationMs: number
    message?: string
    details?: unknown
    error?: MonaApiSerializedError
    diagnostics?: MonaApiTestSession["diagnostics"]
}

export interface MonaApiTestRunOptions extends MonaApiSuiteDiscoveryOptions {
    suiteIds?: readonly string[]
    sessionOptions?: MonaApiTestOptions
    /** Continue with other cases and suites after a failure. Defaults to true. */
    continueOnError?: boolean
}

export interface MonaApiTestRunSummary {
    total: number
    passed: number
    failed: number
    byType: Partial<Record<MonaApiTestResultType, number>>
    bySeverity: Partial<Record<MonaApiTestResultSeverity, number>>
}

export interface MonaApiTestRunReport {
    startedAt: string
    finishedAt: string
    discovery: MonaApiSuiteDiscoveryReport
    results: MonaApiTestResult[]
    summary: MonaApiTestRunSummary
}

export function serializeMonaApiTestError(error: unknown): MonaApiSerializedError {
    if (error && typeof error === "object") {
        const candidate = error as Record<string, unknown>
        return {
            name: typeof candidate.name === "string" ? candidate.name : "Error",
            message: typeof candidate.message === "string" ? candidate.message : String(error),
            code: typeof candidate.code === "string" ? candidate.code : undefined,
            details: candidate.details,
            stack: typeof candidate.stack === "string" ? candidate.stack : undefined
        }
    }
    return {name: "Error", message: String(error)}
}

// Project-facing names. Mona API remains the browser control transport, while
// suites describe project behavior rather than testing the transport itself.
export type ProjectTestCaseCategory = MonaApiTestCaseCategory
export type ProjectTestCaseRunner = MonaApiTestCaseRunner
export type ProjectTestCaseDefinition = MonaApiTestCaseDefinition
export type ProjectTestFailureType = MonaApiTestFailureType
export type ProjectTestSuiteDefinition = MonaApiTestSuiteDefinition
export type ProjectTestSuite = MonaApiTestSuite
export type ProjectDiscoveredSuite = MonaApiDiscoveredSuite
export type ProjectDiscoveryIssueType = MonaApiDiscoveryIssueType
export type ProjectDiscoveryIssue = MonaApiDiscoveryIssue
export type ProjectSuiteDiscoveryOptions = MonaApiSuiteDiscoveryOptions
export type ProjectSuiteDiscoveryReport = MonaApiSuiteDiscoveryReport
export type ProjectTestResult = MonaApiTestResult
export type ProjectTestResultSeverity = MonaApiTestResultSeverity
export type ProjectTestResultType = MonaApiTestResultType
export type ProjectTestRunOptions = MonaApiTestRunOptions
export type ProjectTestRunReport = MonaApiTestRunReport
export type ProjectTestRunSummary = MonaApiTestRunSummary

export const ProjectSuiteDefinitionError = MonaApiSuiteDefinitionError
export const validateProjectTestSuite = validateMonaApiTestSuite
export const serializeProjectTestError = serializeMonaApiTestError

export class ProjectTestFailure extends MonaApiTestFailure {
    constructor(resultType: ProjectTestFailureType, message: string, details?: unknown) {
        super(resultType, message, details)
        this.name = "ProjectTestFailure"
    }
}

export function defineProjectTestSuite(input: ProjectTestSuiteDefinition): ProjectTestSuite {
    return defineMonaApiTestSuite(input)
}
