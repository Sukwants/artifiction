import assert from "node:assert/strict"
import {test} from "node:test"
import {DEFAULT_OPTIMIZATION_ALGORITHM, optimizationAlgorithmChoice, resolveOptimizationAlgorithm} from "../../src/utils/optimizationAlgorithm"
import {useComputeConstraint} from "../../src/composables/constraint"

test("stored algorithm identifiers remain valid; absent or unknown values use the default", () => {
    for (const value of ["AStar", "Heuristic", "Naive"] as const) {
        assert.equal(resolveOptimizationAlgorithm(value), value)
    }
    for (const value of [undefined, null, "", "future-algorithm", 0, {}]) {
        assert.equal(resolveOptimizationAlgorithm(value), DEFAULT_OPTIMIZATION_ALGORITHM)
    }
})

test("legacy selection displays grouped search without rewriting its persisted identifier", () => {
    const state = useComputeConstraint()
    assert.equal(state.algorithm.value, DEFAULT_OPTIMIZATION_ALGORITHM)
    state.algorithm.value = "Heuristic"
    assert.equal(state.algorithmChoice.value, "AStar")
    assert.equal(state.algorithm.value, "Heuristic")
    state.algorithmChoice.value = "Naive"
    assert.equal(state.algorithm.value, "Naive")
    state.algorithmChoice.value = "AStar"
    assert.equal(state.algorithm.value, "AStar")
    assert.equal(optimizationAlgorithmChoice("Naive"), "Naive")
})
