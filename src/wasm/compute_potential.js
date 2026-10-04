import { deepCopy } from "../utils/common"
import { requestWorker } from "./worker_request"

export function wasmComputeArtifactPotential(potentialFunctionInterface, artifacts, timeout = 120000) {
    return requestWorker(
        () => new Worker(new URL("@worker/compute_potential.worker.js", import.meta.url)),
        () => ({potentialFunctionInterface: deepCopy(potentialFunctionInterface), artifacts}),
        timeout
    )
}
