import {deepCopy} from "@/utils/common"

export interface AnalysisNode {
    key: string
    kind: "group" | "number" | "string" | "boolean" | "null" | "value"
    value?: unknown
    children?: AnalysisNode[]
}

function toNode(key: string, value: any): AnalysisNode {
    if (value === null || value === undefined) return {key, kind: "null", value: null}
    if (typeof value === "number") return {key, kind: "number", value}
    if (typeof value === "string") return {key, kind: "string", value}
    if (typeof value === "boolean") return {key, kind: "boolean", value}
    if (Array.isArray(value)) {
        return {key, kind: "group", children: value.map((item, index) => toNode(String(index), item))}
    }
    if (typeof value === "object") {
        return {key, kind: "group", children: Object.entries(value).map(([name, item]) => toNode(name, item))}
    }
    return {key, kind: "value", value: deepCopy(value)}
}

function isDamageResult(value: any): boolean {
    return value && typeof value === "object"
        && typeof value.expectation === "number"
        && typeof value.critical === "number"
        && typeof value.non_critical === "number"
}

function collectVariants(value: any, path: string[] = []): any[] {
    if (!value || typeof value !== "object") return []
    if (isDamageResult(value.result)) {
        const details = Object.entries(value)
            .filter(([key]) => key !== "result")
            .map(([key, item]) => toNode(key, item))
        return [{
            key: path.join(".") || "default",
            result: {
                expectation: value.result.expectation,
                critical: value.result.critical,
                nonCritical: value.result.non_critical,
            },
            details,
        }]
    }

    const result: any[] = []
    for (const [key, item] of Object.entries(value)) {
        result.push(...collectVariants(item, [...path, key]))
    }
    return result
}

export function normalizeDamageEvent(raw: any) {
    const eventKey = raw && typeof raw === "object" ? Object.keys(raw)[0] ?? "None" : "None"
    return {
        kind: eventKey,
        variants: collectVariants(raw),
        raw: deepCopy(raw),
    }
}
