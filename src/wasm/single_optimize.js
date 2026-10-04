import { requestWorker } from "./worker_request"

export function wasmSingleOptimize(optimizeConfig, artifacts, timeout = 600000) {
    return requestWorker(
        () => new Worker(new URL("@worker/optimize_artifact.js", import.meta.url)),
        () => ({optimizeConfig, artifacts}),
        timeout,
        () => "计算发生错误"
    )
}
