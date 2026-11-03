import assert from "node:assert/strict"
import {setImmediate} from "node:timers/promises"
import {test} from "node:test"
import localforage from "localforage"
import backend from "../../src/store/backend"

function deferred() {
    let resolve!: () => void
    let reject!: (error: Error) => void
    const promise = new Promise<void>((done, fail) => { resolve = done; reject = fail })
    return {promise, resolve, reject}
}

test("backend readiness waits for browser data and metadata writes", async context => {
    const data = deferred()
    const metadata = deferred()
    const dataStarted = deferred()
    const metadataStarted = deferred()
    context.mock.method(localforage, "setItem", async (key: string, value: unknown) => {
        if (key === "backend_meta") {
            metadataStarted.resolve()
            await metadata.promise
        } else {
            dataStarted.resolve()
            await data.promise
        }
        return value
    })
    const write = backend.setItem("mona_account_presets_1", {algorithm: "Naive"})
    let ready = false
    const waiting = backend.allReady().then(() => { ready = true })
    try {
        await dataStarted.promise
        await setImmediate()
        assert.equal(ready, false, "readiness must include the browser database")
        data.resolve()
        await metadataStarted.promise
        await setImmediate()
        assert.equal(ready, false, "metadata must finish before readiness resolves")
    } finally {
        data.resolve()
        metadata.resolve()
        await Promise.all([write, waiting])
    }
})

test("backend readiness also drains writes added while waiting", async context => {
    const first = deferred()
    const second = deferred()
    context.mock.method(localforage, "setItem", async (key: string, value: unknown) => {
        if (key === "first") await first.promise
        if (key === "second") await second.promise
        return value
    })
    const firstWrite = backend.setItem("first", 1)
    let ready = false
    const waiting = backend.allReady().then(() => { ready = true })
    const secondWrite = backend.setItem("second", 2)
    try {
        first.resolve()
        await firstWrite
        await setImmediate()
        assert.equal(ready, false, "a later write must not be left pending")
    } finally {
        first.resolve()
        second.resolve()
        await Promise.all([firstWrite, secondWrite, waiting])
    }
})

test("backend readiness waits for browser deletions", async context => {
    const deletion = deferred()
    context.mock.method(localforage, "removeItem", () => deletion.promise)
    context.mock.method(localforage, "setItem", async (_key: string, value: unknown) => value)
    const write = backend.removeItem("mona_account_presets_2")
    let ready = false
    const waiting = backend.allReady().then(() => { ready = true })
    try {
        await setImmediate()
        assert.equal(ready, false)
    } finally {
        deletion.resolve()
        await Promise.all([write, waiting])
    }
})

for (const operation of ["setItem", "removeItem"] as const) {
    test(`backend readiness reports a failed ${operation} and releases it`, async context => {
        const failure = deferred()
        context.mock.method(localforage, operation, () => failure.promise)
        const write = operation === "setItem" ? backend.setItem("failed", 1) : backend.removeItem("failed")
        const writeRejected = assert.rejects(write, /storage failed/)
        const readyRejected = assert.rejects(backend.allReady(), /storage failed/)
        failure.reject(new Error("storage failed"))
        await Promise.all([writeRejected, readyRejected])
        await backend.allReady()
    })
}
