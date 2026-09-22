import assert from "node:assert/strict"
import {withMonaApiTest} from "../framework"

/**
 * This file demonstrates the public shape of a project browser test.
 * It is intentionally not named *.test.ts, so it is not executed automatically.
 */
export async function runSmokeTest() {
    return withMonaApiTest(async ({api, navigate}) => {
        assert.equal(api.version, "1.1.0")

        const capabilities = await api.capabilities()
        assert.equal(capabilities.calculator, true)

        const route = await navigate("calculator")
        assert.equal(route.page, "calculator")

        return {version: api.version, route}
    }, {name: "project-smoke"})
}
