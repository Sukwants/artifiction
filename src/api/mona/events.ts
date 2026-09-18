import {deepCopy} from "@/utils/common"

export interface MonaApiEvent {
    type: string
    revision: number
    source: "api" | "page"
    data?: unknown
}

type Listener = (event: MonaApiEvent) => void

const listeners = new Set<Listener>()
let revision = 0

export function emitMonaApiEvent(type: string, data?: unknown, source: "api" | "page" = "api") {
    const event: MonaApiEvent = {
        type,
        revision: ++revision,
        source,
        data: data === undefined ? undefined : deepCopy(data),
    }
    for (const listener of listeners) {
        try {
            listener(event)
        } catch (error) {
            console.error("monaApi event listener failed", error)
        }
    }
    return event
}

export const monaApiEvents = {
    subscribe(listener: Listener) {
        listeners.add(listener)
        return () => listeners.delete(listener)
    },
    getRevision() {
        return revision
    },
}
