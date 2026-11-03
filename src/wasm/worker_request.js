function formatWorkerError(error) {
    return "计算发生错误：" + (error?.message ?? String(error))
}

export function requestWorker(createWorker, createRequest, timeout, formatError = formatWorkerError) {
    return new Promise((resolve, reject) => {
        let worker
        let timer
        let settled = false
        let sent = false

        function finish(callback, value) {
            if (settled) return
            settled = true
            clearTimeout(timer)
            if (worker) {
                worker.onmessage = null
                worker.onerror = null
                worker.onmessageerror = null
                worker.terminate()
            }
            callback(value)
        }

        const fail = error => finish(reject, formatError(error))

        try {
            const request = createRequest()
            worker = createWorker()
            worker.onmessage = event => {
                if (settled) return
                const data = event.data
                if (data?.type === "ready" && !sent) {
                    sent = true
                    try {
                        worker.postMessage(request)
                    } catch (error) {
                        fail(error)
                    }
                } else if (data?.type === "result") {
                    finish(resolve, data.result)
                } else if (data?.type === "error") {
                    fail(new Error(data.message || "未知错误"))
                }
            }
            worker.onerror = fail
            worker.onmessageerror = () => fail(new Error("计算结果无法读取"))
            timer = setTimeout(() => finish(reject, "计算超时"), timeout)
        } catch (error) {
            fail(error)
        }
    })
}
