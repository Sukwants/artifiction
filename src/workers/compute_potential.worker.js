async function initWasm() {
    const mona = await import("mona")

    self.onmessage = function (e) {
        try {
            const pf = e.data.potentialFunctionInterface
            const artifacts = e.data.artifacts
            const result = mona.PotentialInterface.get_potential(artifacts, pf)
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
