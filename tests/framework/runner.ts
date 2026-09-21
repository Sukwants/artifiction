import {MonaApiRemoteError, withMonaApiTest, type MonaApiTestDiagnostics} from "../mona-api/framework"
import {discoverMonaApiTestSuites} from "./discovery"
import {
    MonaApiTestFailure,
    serializeMonaApiTestError,
    type MonaApiDiscoveredSuite,
    type MonaApiTestCaseDefinition,
    type MonaApiTestResult,
    type MonaApiTestResultSeverity,
    type MonaApiTestResultType,
    type MonaApiTestRunOptions,
    type MonaApiTestRunReport,
    type MonaApiTestRunSummary
} from "./registry"

function isTimeoutError(error: unknown): boolean {
    return error instanceof Error && /timeout|timed out/i.test(error.message)
}

function isAssertionError(error: unknown): boolean {
    return error instanceof Error && error.name === "AssertionError"
}

function isPageCrash(diagnostics: MonaApiTestDiagnostics, initialCrashCount: number): boolean {
    return diagnostics.pageCrashes.length > initialCrashCount
}

function isCalculationError(error: unknown, testCase: MonaApiTestCaseDefinition): boolean {
    if (testCase.category === "calculation") return true
    return error instanceof MonaApiRemoteError && Boolean(error.code?.startsWith("COMPUTATION_"))
}

function resultSeverity(type: MonaApiTestResultType): MonaApiTestResultSeverity {
    if (type === "passed") return "success"
    if (type === "page-crashed" || type === "suite-import-error" || type === "framework-error") return "fatal"
    if (type === "test-root-missing" || type === "test-file-missing" || type === "suite-invalid") return "error"
    return "error"
}

function createResult(
    suite: MonaApiDiscoveredSuite,
    input: Pick<MonaApiTestResult, "type" | "durationMs"> & Partial<MonaApiTestResult>
): MonaApiTestResult {
    return {
        suiteId: suite.suiteId,
        suiteName: suite.suite.name,
        type: input.type,
        severity: resultSeverity(input.type),
        ok: input.type === "passed",
        durationMs: input.durationMs,
        caseId: input.caseId,
        caseName: input.caseName,
        message: input.message,
        details: input.details,
        error: input.error,
        diagnostics: input.diagnostics
    }
}

function classifyFailure(
    error: unknown,
    testCase: MonaApiTestCaseDefinition,
    diagnostics: MonaApiTestDiagnostics,
    initialCrashCount: number
): MonaApiTestResultType {
    if (isPageCrash(diagnostics, initialCrashCount)) return "page-crashed"
    if (error instanceof MonaApiTestFailure) return error.resultType
    if (isCalculationError(error, testCase)) return "calculation-error"
    if (isTimeoutError(error)) return "timeout"
    if (isAssertionError(error)) return "assertion-error"
    return "test-failed"
}

function copyDiagnostics(diagnostics: MonaApiTestDiagnostics): MonaApiTestDiagnostics {
    return {
        console: [...diagnostics.console],
        pageErrors: [...diagnostics.pageErrors],
        pageCrashes: [...diagnostics.pageCrashes],
        failedRequests: [...diagnostics.failedRequests]
    }
}

async function waitForDebugKey(suiteName: string): Promise<void> {
    const stdin = process.stdin
    const isTerminal = Boolean(stdin.isTTY && stdin.setRawMode)
    const previousRawMode = stdin.isRaw

    process.stdout.write(`[debug] ${suiteName} 已完成，浏览器保持打开。按任意键继续...`)
    await new Promise<void>(resolve => {
        const finish = () => {
            stdin.off("data", onData)
            if (isTerminal && previousRawMode !== undefined) stdin.setRawMode(previousRawMode)
            stdin.pause()
            resolve()
        }
        const onData = () => finish()

        if (isTerminal) stdin.setRawMode(true)
        stdin.resume()
        stdin.once("data", onData)
    })
    process.stdout.write("\n")
}

async function runSuite(
    discovered: MonaApiDiscoveredSuite,
    options: MonaApiTestRunOptions
): Promise<MonaApiTestResult[]> {
    const results: MonaApiTestResult[] = []
    const continueOnError = options.continueOnError ?? true
    const suiteOptions = {
        ...options.sessionOptions,
        ...discovered.suite.options,
        name: discovered.suite.options?.name ?? options.sessionOptions?.name ?? discovered.suite.name
    }

    try {
        await withMonaApiTest(async session => {
            for (const testCase of discovered.suite.cases) {
                const startedAt = Date.now()
                const initialCrashCount = session.diagnostics.pageCrashes.length
                try {
                    const details = await testCase.run(session)
                    results.push(createResult(discovered, {
                        type: "passed",
                        durationMs: Date.now() - startedAt,
                        caseId: testCase.id,
                        caseName: testCase.name,
                        details
                    }))
                } catch (error) {
                    const type = classifyFailure(error, testCase, session.diagnostics, initialCrashCount)
                    results.push(createResult(discovered, {
                        type,
                        durationMs: Date.now() - startedAt,
                        caseId: testCase.id,
                        caseName: testCase.name,
                        message: error instanceof Error ? error.message : String(error),
                        details: error instanceof MonaApiTestFailure ? error.details : undefined,
                        error: serializeMonaApiTestError(error),
                        diagnostics: copyDiagnostics(session.diagnostics)
                    }))
                    await session.saveFailureArtifacts(error)
                    if (!continueOnError) break
                }
            }
            if (suiteOptions.debug) await waitForDebugKey(discovered.suite.name)
        }, suiteOptions)
    } catch (error) {
        results.push(createResult(discovered, {
            type: "framework-error",
            durationMs: 0,
            message: error instanceof Error ? error.message : String(error),
            error: serializeMonaApiTestError(error)
        }))
    }

    return results
}

function discoveryIssueResult(issue: {
    type: "test-root-missing" | "test-file-missing" | "suite-import-error" | "suite-invalid"
    suiteId: string
    message: string
    error?: ReturnType<typeof serializeMonaApiTestError>
}): MonaApiTestResult {
    const type: MonaApiTestResultType = issue.type
    return {
        suiteId: issue.suiteId,
        type,
        severity: resultSeverity(type),
        ok: false,
        durationMs: 0,
        message: issue.message,
        error: issue.error
    }
}

function summarize(results: MonaApiTestResult[]): MonaApiTestRunSummary {
    const byType: MonaApiTestRunSummary["byType"] = {}
    const bySeverity: MonaApiTestRunSummary["bySeverity"] = {}
    for (const result of results) {
        byType[result.type] = (byType[result.type] ?? 0) + 1
        bySeverity[result.severity] = (bySeverity[result.severity] ?? 0) + 1
    }
    return {
        total: results.length,
        passed: results.filter(result => result.ok).length,
        failed: results.filter(result => !result.ok).length,
        byType,
        bySeverity
    }
}

export async function runMonaApiTestSuites(
    options: MonaApiTestRunOptions = {}
): Promise<MonaApiTestRunReport> {
    const startedAt = new Date().toISOString()
    const discovery = await discoverMonaApiTestSuites({
        ...options,
        include: options.suiteIds ?? options.include
    })
    const results = discovery.issues.map(discoveryIssueResult)
    const selectedSuites = options.suiteIds
        ? discovery.suites.filter(item => options.suiteIds?.includes(item.suiteId))
        : discovery.suites
    const continueOnError = options.continueOnError ?? true

    for (const suite of selectedSuites) {
        if (!continueOnError && results.some(result => !result.ok)) break
        results.push(...await runSuite(suite, options))
    }

    return {
        startedAt,
        finishedAt: new Date().toISOString(),
        discovery,
        results,
        summary: summarize(results)
    }
}

export const runProjectTestSuites = runMonaApiTestSuites
