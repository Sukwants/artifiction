async function init_wasm_in_worker() {
    const mona = await import("mona")

    self.onmessage = function (e) {
        try {
            const optimizeConfig = e.data.optimizeConfig
            const artifacts = e.data.artifacts
            const result = mona.OptimizeSingleWasm.optimize(optimizeConfig, artifacts)
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

init_wasm_in_worker().catch(reportError)
