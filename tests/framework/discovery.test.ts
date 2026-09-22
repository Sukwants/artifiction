import assert from "node:assert/strict"
import {test} from "node:test"
import {discoverProjectTestSuites} from "./index"

test("discovers suites from tests/suites", async () => {
    const report = await discoverProjectTestSuites()
    assert.ok(report.suites.some(item => item.suiteId === "project-smoke"))
    assert.equal(report.issues.length, 0)
})
