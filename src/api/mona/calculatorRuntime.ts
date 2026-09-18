import {MonaApiError} from "./errors"

export interface CalculatorCharacterController {
    getState(): any
    patchState(patch: any): Promise<any>
    listConfigs(): any[]
    getConfig(id: string, mode?: "local" | "effective"): any
    setConfig(id: string, value: unknown, mode?: "local" | "effective"): any
    setConfigUnlinked(id: string, unlinked: boolean): void
    getCurrentDamage(): any
    getTransformativeDamage(): Record<string, any>
    getElevativeDamage(): Record<string, any>
    getPanel(): any
    applyPreset(name: string): Promise<any>
    optimize(options?: { applyResult?: number | false }): Promise<any>
}

export interface CalculatorPageController {
    listCharacters(): any[]
    getCharacter(id: number): CalculatorCharacterController | undefined
    addCharacter(input?: any): Promise<any>
    removeCharacter(id: number): Promise<void>
    getSelectedCharacterId(): number | null
    selectCharacter(id: number): Promise<void>
    patchCharacter(id: number, patch: any): Promise<any>
    getState(): any
    setState(state: any, mode?: "replace" | "merge"): Promise<any>
}

let currentController: CalculatorPageController | null = null
const controllerReadyListeners = new Set<() => void>()

export function registerCalculatorController(controller: CalculatorPageController) {
    currentController = controller
    for (const listener of controllerReadyListeners) listener()
    controllerReadyListeners.clear()
    return () => {
        if (currentController === controller) currentController = null
    }
}

export function getCalculatorController(): CalculatorPageController {
    if (!currentController) {
        throw new MonaApiError("PAGE_NOT_ACTIVE", "Calculator page is not active")
    }
    return currentController
}

export function isCalculatorActive() {
    return currentController !== null
}

export function waitForCalculatorActive(timeoutMs = 10000) {
    if (currentController) return Promise.resolve()

    return new Promise<void>((resolve, reject) => {
        const listener = () => {
            window.clearTimeout(timer)
            resolve()
        }
        const timer = window.setTimeout(() => {
            controllerReadyListeners.delete(listener)
            reject(new MonaApiError("PAGE_NOT_READY", "Calculator page did not become ready in time"))
        }, timeoutMs)
        controllerReadyListeners.add(listener)
    })
}
