import assert from "node:assert/strict"
import {test} from "node:test"
import {
    defineProjectTestSuite,
    validateProjectTestSuite
} from "./index"

test("test suite registration validates case ids", () => {
    assert.deepEqual(
        validateProjectTestSuite({
            id: "invalid",
            cases: [
                {id: "same", run: async () => undefined},
                {id: "same", run: async () => undefined}
            ]
        }),
        ["duplicate case id: same"]
    )
})

test("test suite registration normalizes names", () => {
    const suite = defineProjectTestSuite({
        id: "example",
        cases: [{id: "case-1", run: async () => undefined}]
    })
    assert.equal(suite.name, "example")
    assert.equal(suite.cases[0].name, "case-1")
})
