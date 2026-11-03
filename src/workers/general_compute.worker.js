import {useMona} from "../wasm/mona"

async function init_wasm_in_worker() {
    const mona = await useMona()

    self.onmessage = function (e) {
        try {
            const dispatch = e.data.dispatch
            const args = e.data.args

            let result = null
            if (dispatch === "best_artifact_set") {
                result = mona.CalcArtifactBestSet.calc_artifact_best_set(...args)
            }
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
