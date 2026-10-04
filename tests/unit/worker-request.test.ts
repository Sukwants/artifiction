import assert from "node:assert/strict"
import {test, type TestContext} from "node:test"
import {requestWorker} from "../../src/wasm/worker_request"
import {wasmCalcBestArtifactSet} from "../../src/wasm/calc_best_artifact_set"
import {wasmComputeArtifactPotential} from "../../src/wasm/compute_potential"
import {wasmSingleOptimize} from "../../src/wasm/single_optimize"
import {team_optimize} from "../../src/wasm/team_optimize"

class FakeWorker {
    onmessage: ((event: {data: unknown}) => void) | null = null
    onerror: ((event: {message: string}) => void) | null = null
    onmessageerror: (() => void) | null = null
    requests: unknown[] = []
    terminations = 0
    sendError: Error | null = null

    postMessage(request: unknown) {
        if (this.sendError) throw this.sendError
        this.requests.push(request)
    }

    terminate() {
        this.terminations++
    }

    receive(data: unknown) {
        this.onmessage?.({data})
    }
}

function mockTimers(context: TestContext) {
    context.mock.timers.enable({apis: ["setTimeout"]})
    return context.mock.method(globalThis, "clearTimeout")
}

function assertReleased(worker: FakeWorker) {
    assert.equal(worker.terminations, 1)
    assert.equal(worker.onmessage, null)
    assert.equal(worker.onerror, null)
    assert.equal(worker.onmessageerror, null)
}

test("worker sends once after readiness and releases resources after returning the result", async context => {
    const clearTimer = mockTimers(context)
    const worker = new FakeWorker()
    const request = {input: "config"}
    const result = [{value: 42}]
    const promise = requestWorker(() => worker, () => request, 50)
    const lateMessage = worker.onmessage!

    worker.receive({type: "progress"})
    assert.equal(worker.requests.length, 0)
    worker.receive({type: "ready"})
    worker.receive({type: "ready"})
    assert.deepEqual(worker.requests, [request])
    worker.receive({type: "result", result})

    assert.equal(await promise, result)
    assert.equal(clearTimer.mock.callCount(), 1)
    assertReleased(worker)
    lateMessage({data: {type: "ready"}})
    lateMessage({data: {type: "result", result: null}})
    context.mock.timers.tick(50)
    assert.equal(worker.requests.length, 1)
    assert.equal(worker.terminations, 1)
})

for (const failure of ["runtime", "initialization", "send", "message"] as const) {
    test(`worker rejects and releases resources on ${failure} failure`, async context => {
        const clearTimer = mockTimers(context)
        const worker = new FakeWorker()
        const promise = requestWorker(() => worker, () => ({}), 50)

        if (failure === "runtime") worker.onerror!({message: "runtime failed"})
        if (failure === "initialization") worker.receive({type: "error", message: "WASM unavailable"})
        if (failure === "send") {
            worker.sendError = new Error("Cannot clone input")
            worker.receive({type: "ready"})
        }
        if (failure === "message") worker.onmessageerror!()

        const expected = {
            runtime: "计算发生错误：runtime failed",
            initialization: "计算发生错误：WASM unavailable",
            send: "计算发生错误：Cannot clone input",
            message: "计算发生错误：计算结果无法读取"
        }[failure]
        await assert.rejects(promise, (error: unknown) => error === expected)
        assert.equal(clearTimer.mock.callCount(), 1)
        assertReleased(worker)
        context.mock.timers.tick(50)
        assert.equal(worker.terminations, 1)
    })
}

test("worker times out before readiness and releases resources", async context => {
    const clearTimer = mockTimers(context)
    const worker = new FakeWorker()
    const promise = requestWorker(() => worker, () => ({}), 50)
    context.mock.timers.tick(50)
    await assert.rejects(promise, (error: unknown) => error === "计算超时")
    assert.equal(clearTimer.mock.callCount(), 1)
    assertReleased(worker)
    assert.equal(worker.requests.length, 0)
})

test("request preparation failure rejects before allocating a worker", async () => {
    let created = false
    const promise = requestWorker(() => {
        created = true
        return new FakeWorker()
    }, () => {
        throw new Error("Cannot read input")
    }, 50)
    await assert.rejects(promise, (error: unknown) => error === "计算发生错误：Cannot read input")
    assert.equal(created, false)
})

test("worker construction failure is a rejected promise", async () => {
    const promise = requestWorker(() => {
        throw new Error("Worker unavailable")
    }, () => ({}), 50)
    await assert.rejects(promise, (error: unknown) => error === "计算发生错误：Worker unavailable")
})

function installWorker(context: TestContext, worker: FakeWorker) {
    const descriptor = Object.getOwnPropertyDescriptor(globalThis, "Worker")
    Object.defineProperty(globalThis, "Worker", {
        configurable: true,
        value: class {
            constructor() {
                return worker
            }
        }
    })
    context.after(() => {
        if (descriptor) Object.defineProperty(globalThis, "Worker", descriptor)
        else Reflect.deleteProperty(globalThis, "Worker")
    })
}

test("public calculation wrappers retain request fields and return values", async context => {
    const input = {config: {value: 1}}
    const artifacts = [{id: 123}]
    const wrappers = [
        {name: "best set", invoke: () => wasmCalcBestArtifactSet(input), request: {args: [input], dispatch: "best_artifact_set"}},
        {name: "potential", invoke: () => wasmComputeArtifactPotential(input, artifacts), request: {potentialFunctionInterface: input, artifacts}},
        {name: "single", invoke: () => wasmSingleOptimize(input, artifacts), request: {optimizeConfig: input, artifacts}},
        {name: "team", invoke: () => team_optimize(input, artifacts), request: {input, artifacts}}
    ]
    for (const wrapper of wrappers) {
        await context.test(wrapper.name, async child => {
            mockTimers(child)
            const worker = new FakeWorker()
            installWorker(child, worker)
            const promise = wrapper.invoke()
            worker.receive({type: "ready"})
            assert.deepEqual(worker.requests, [wrapper.request])
            const result = {artifacts: [123]}
            worker.receive({type: "result", result})
            assert.equal(await promise, result)
            assertReleased(worker)
        })
    }
})

test("single optimization keeps its existing error text and clears its timer", async context => {
    const clearTimer = mockTimers(context)
    const worker = new FakeWorker()
    installWorker(context, worker)
    const promise = wasmSingleOptimize({}, [])
    worker.onerror!({message: "WASM failed"})
    await assert.rejects(promise, (error: unknown) => error === "计算发生错误")
    assert.equal(clearTimer.mock.callCount(), 1)
    assertReleased(worker)
})

test("team optimization keeps error events and releases the worker", async context => {
    mockTimers(context)
    const worker = new FakeWorker()
    installWorker(context, worker)
    const promise = team_optimize({}, [], 50)
    const error = {message: "WASM failed"}
    worker.onerror!(error)
    await assert.rejects(promise, (received: unknown) => received === error)
    assertReleased(worker)
})

test("team optimization supports a timeout", async context => {
    mockTimers(context)
    const worker = new FakeWorker()
    installWorker(context, worker)
    const promise = team_optimize({}, [], 50)
    context.mock.timers.tick(50)
    await assert.rejects(promise, (received: unknown) => received === "计算超时")
    assertReleased(worker)
})
