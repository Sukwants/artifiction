import assert from "node:assert/strict"
import {test} from "node:test"
import {resolveBrowserExecutablePath} from "./framework"

test("browser path errors explain how to configure puppeteer-core", () => {
    assert.throws(
        () => resolveBrowserExecutablePath("C:/browser-that-does-not-exist.exe"),
        /Configured browser executable does not exist/
    )
})
