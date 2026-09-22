import {readdir} from "node:fs/promises"
import {access} from "node:fs/promises"
import {join, resolve} from "node:path"
import {pathToFileURL} from "node:url"
import {
    defineMonaApiTestSuite,
    serializeMonaApiTestError,
    type MonaApiDiscoveryIssue,
    type MonaApiDiscoveredSuite,
    type MonaApiTestSuiteDefinition,
    type MonaApiSuiteDiscoveryOptions,
    type MonaApiSuiteDiscoveryReport
} from "./registry"

const DEFAULT_ENTRY_FILE = "suite.ts"

function defaultRootDir(): string {
    return resolve(process.cwd(), "tests", "suites")
}

function isSelectedSuite(
    suiteId: string,
    options: Pick<MonaApiSuiteDiscoveryOptions, "include" | "exclude">
): boolean {
    if (options.include && options.include.length > 0 && !options.include.includes(suiteId)) return false
    if (options.exclude?.includes(suiteId)) return false
    return true
}

function issue(
    type: MonaApiDiscoveryIssue["type"],
    suiteId: string,
    directory: string,
    entryFile: string,
    message: string,
    error?: unknown
): MonaApiDiscoveryIssue {
    return {
        type,
        suiteId,
        directory,
        entryFile,
        message,
        error: error === undefined ? undefined : serializeMonaApiTestError(error)
    }
}

async function isFile(path: string): Promise<boolean> {
    try {
        await access(path)
        return true
    } catch {
        return false
    }
}

function getSuiteExport(module: Record<string, unknown>): unknown {
    return module.default ?? module.suite
}

export async function discoverMonaApiTestSuites(
    inputOptions: MonaApiSuiteDiscoveryOptions = {}
): Promise<MonaApiSuiteDiscoveryReport> {
    const rootDir = resolve(inputOptions.rootDir ?? defaultRootDir())
    const entryFile = inputOptions.entryFile ?? DEFAULT_ENTRY_FILE
    const suites: MonaApiDiscoveredSuite[] = []
    const issues: MonaApiDiscoveryIssue[] = []

    let directories
    try {
        directories = await readdir(rootDir, {withFileTypes: true})
    } catch (error) {
        issues.push(issue(
            "test-root-missing",
            "*",
            rootDir,
            join(rootDir, entryFile),
            `Test suite root does not exist: ${rootDir}`,
            error
        ))
        return {rootDir, suites, issues}
    }

    for (const directoryEntry of directories
        .filter(entry => entry.isDirectory())
        .sort((left, right) => left.name.localeCompare(right.name))) {
        const suiteId = directoryEntry.name
        if (!isSelectedSuite(suiteId, inputOptions)) continue

        const directory = join(rootDir, suiteId)
        const suiteEntry = join(directory, entryFile)
        if (!await isFile(suiteEntry)) {
            issues.push(issue(
                "test-file-missing",
                suiteId,
                directory,
                suiteEntry,
                `Test suite entry is missing: ${suiteEntry}`
            ))
            continue
        }

        try {
            const module = await import(pathToFileURL(suiteEntry).href) as Record<string, unknown>
            const candidate = getSuiteExport(module)
            const suite = defineMonaApiTestSuite(candidate as MonaApiTestSuiteDefinition)
            const validationIssues = suite.id === suiteId
                ? []
                : [`suite id must match directory name '${suiteId}'`]
            if (validationIssues.length > 0) {
                issues.push(issue(
                    "suite-invalid",
                    suiteId,
                    directory,
                    suiteEntry,
                    validationIssues.join("; ")
                ))
                continue
            }
            suites.push({suite, suiteId, directory, entryFile: suiteEntry})
        } catch (error) {
            const isDefinitionError = error instanceof Error && error.name === "MonaApiSuiteDefinitionError"
            issues.push(issue(
                isDefinitionError ? "suite-invalid" : "suite-import-error",
                suiteId,
                directory,
                suiteEntry,
                isDefinitionError ? error.message : `Failed to import test suite: ${suiteEntry}`,
                error
            ))
        }
    }

    return {rootDir, suites, issues}
}

export const discoverProjectTestSuites = discoverMonaApiTestSuites
