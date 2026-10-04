async function initWasm() {
    const mona = await import("mona")

    self.onmessage = function (e) {
        try {
            const input = e.data.input
            const artifacts = e.data.artifacts
            const result = mona.TeamOptimizationWasm.optimize_team2(input, artifacts)
            self.postMessage({type: "result", result})
        } catch (error) {
            reportError(error)
        }
    }

    self.postMessage({
        type: "ready"
    })
}

function reportError(error) {
    self.postMessage({type: "error", message: error?.message ?? String(error)})
}

initWasm().catch(reportError)
