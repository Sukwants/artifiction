import { deepCopy } from "@/utils/common"
import { requestWorker } from "./worker_request"

export function wasmCalcBestArtifactSet(calcInterface, timeout = 120000) {
    return requestWorker(
        () => new Worker(new URL("@worker/general_compute.worker.js", import.meta.url)),
        () => ({args: [deepCopy(calcInterface)], dispatch: "best_artifact_set"}),
        timeout
    )
}
