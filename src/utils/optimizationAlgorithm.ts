import type {PresetAlgorithm} from "@/types/preset"

// These are persisted identifiers. Display names may change; stored names must not.
export const DEFAULT_OPTIMIZATION_ALGORITHM: PresetAlgorithm = "AStar"
export type OptimizationAlgorithmChoice = "AStar" | "Naive"

export function resolveOptimizationAlgorithm(value: unknown): PresetAlgorithm {
    return value === "AStar" || value === "Naive" || value === "Heuristic"
        ? value
        : DEFAULT_OPTIMIZATION_ALGORITHM
}

export function optimizationAlgorithmChoice(value: unknown): OptimizationAlgorithmChoice {
    return resolveOptimizationAlgorithm(value) === "Naive" ? "Naive" : "AStar"
}
