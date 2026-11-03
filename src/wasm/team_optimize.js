import { requestWorker } from "./worker_request"

export function team_optimize(input, artifacts, timeout = 600000) {
    return requestWorker(
        () => new Worker(new URL("@worker/team_optimization.js", import.meta.url)),
        () => ({input, artifacts}),
        timeout,
        error => error
    )
}
