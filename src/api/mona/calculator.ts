import {deepCopy} from "@/utils/common"
import {getCalculatorController, isCalculatorActive} from "./calculatorRuntime"
import {normalizeDamageEvent} from "./damage"
import {MonaApiError, requireValue} from "./errors"
import {emitMonaApiEvent} from "./events"
import {usePresetStore} from "@/store/pinia/preset"
import {wasmCalcBestArtifactSet} from "@/wasm/calc_best_artifact_set"

function character(id: number) {
    return requireValue(getCalculatorController().getCharacter(id), "NOT_FOUND", `Calculator character ${id} does not exist`)
}

function throwComputationError(error: unknown): never {
    if (error instanceof MonaApiError) throw error
    const message = error instanceof Error ? error.message : String(error)
    const code = /timeout|超时/i.test(message) ? "COMPUTATION_TIMEOUT" : "COMPUTATION_FAILED"
    throw new MonaApiError(code, message)
}

export const calculatorApi = {
    isActive: isCalculatorActive,
    characters: {
        async list() { return deepCopy(getCalculatorController().listCharacters()) },
        async get(id: number) { return deepCopy(character(id).getState()) },
        async add(input?: any) {
            const result = await getCalculatorController().addCharacter(input)
            emitMonaApiEvent("calculator.character.created", {id: result.id})
            return deepCopy(result)
        },
        async patch(id: number, patch: any) {
            const result = await getCalculatorController().patchCharacter(id, patch)
            emitMonaApiEvent("calculator.character.changed", {id})
            return deepCopy(result)
        },
        async remove(id: number) {
            await getCalculatorController().removeCharacter(id)
            emitMonaApiEvent("calculator.character.removed", {id})
        },
        async setTeam(id: number, teamId: number) {
            const result = await getCalculatorController().patchCharacter(id, {teamId})
            emitMonaApiEvent("calculator.character.changed", {id})
            return deepCopy(result)
        },
        async setOnField(id: number, onField: boolean) {
            const result = await getCalculatorController().patchCharacter(id, {onField})
            emitMonaApiEvent("calculator.character.changed", {id})
            return deepCopy(result)
        },
        async setArtifacts(id: number, artifacts: any) {
            const result = await getCalculatorController().patchCharacter(id, {artifacts})
            emitMonaApiEvent("calculator.character.changed", {id})
            return deepCopy(result)
        },
        async applyPreset(id: number, name: string) {
            const result = await character(id).applyPreset(name)
            emitMonaApiEvent("calculator.character.changed", {id, preset: name})
            return deepCopy(result)
        },
    },
    selection: {
        async get() { return getCalculatorController().getSelectedCharacterId() },
        async select(id: number) {
            await getCalculatorController().selectCharacter(id)
            emitMonaApiEvent("calculator.selection.changed", {characterId: id})
        },
    },
    state: {
        async get() { return deepCopy(getCalculatorController().getState()) },
        async set(state: any, options: {mode?: "replace" | "merge"} = {}) {
            const result = await getCalculatorController().setState(state, options.mode)
            emitMonaApiEvent("calculator.state.changed")
            return deepCopy(result)
        },
        async patch(patch: any) {
            const current = getCalculatorController().getState()
            const result = await getCalculatorController().setState({...current, ...deepCopy(patch)}, "merge")
            emitMonaApiEvent("calculator.state.changed")
            return deepCopy(result)
        },
    },
    config: {
        async list(query: {characterId?: number, module?: string} = {}) {
            const ids = query.characterId === undefined
                ? getCalculatorController().listCharacters().map(item => item.id)
                : [query.characterId]
            return ids.flatMap(id => character(id).listConfigs())
                .filter(item => query.module === undefined || item.address.module_name === query.module)
                .map(deepCopy)
        },
        async get(id: string, options: {mode?: "local" | "effective"} = {}) {
            const characterId = Number(id.split("#", 1)[0])
            return deepCopy(character(characterId).getConfig(id, options.mode))
        },
        async set(id: string, value: unknown, options: {mode?: "local" | "effective"} = {}) {
            const characterId = Number(id.split("#", 1)[0])
            const result = character(characterId).setConfig(id, value, options.mode)
            emitMonaApiEvent("calculator.config.changed", {id})
            return deepCopy(result)
        },
        async setUnlinked(id: string, unlinked: boolean) {
            const characterId = Number(id.split("#", 1)[0])
            character(characterId).setConfigUnlinked(id, unlinked)
            emitMonaApiEvent("calculator.config.changed", {id, unlinked})
        },
    },
    damage: {
        async getCurrent(input: {characterId: number, includeRaw?: boolean}) {
            const normalized = normalizeDamageEvent(character(input.characterId).getCurrentDamage())
            if (!input.includeRaw) delete (normalized as any).raw
            return normalized
        },
        async getAllTransformative(input: {characterId: number, includeRaw?: boolean}) {
            const raw = character(input.characterId).getTransformativeDamage()
            return Object.fromEntries(Object.entries(raw).map(([key, value]) => {
                const normalized = normalizeDamageEvent(value)
                if (!input.includeRaw) delete (normalized as any).raw
                return [key, normalized]
            }))
        },
        async getTransformative(input: {characterId: number, reaction: string, includeRaw?: boolean}) {
            const raw = character(input.characterId).getTransformativeDamage()
            const value = requireValue(raw[input.reaction], "UNSUPPORTED_REACTION", `Unsupported transformative reaction ${input.reaction}`)
            const normalized = normalizeDamageEvent(value)
            if (!input.includeRaw) delete (normalized as any).raw
            return normalized
        },
        async getAllElevative(input: {characterId: number, includeRaw?: boolean}) {
            const raw = character(input.characterId).getElevativeDamage()
            return Object.fromEntries(Object.entries(raw).map(([key, value]) => {
                const normalized = normalizeDamageEvent(value)
                if (!input.includeRaw) delete (normalized as any).raw
                return [key, normalized]
            }))
        },
        async getElevative(input: {characterId: number, reaction: string, includeRaw?: boolean}) {
            const raw = character(input.characterId).getElevativeDamage()
            const value = requireValue(raw[input.reaction], "UNSUPPORTED_REACTION", `Unsupported elevative reaction ${input.reaction}`)
            const normalized = normalizeDamageEvent(value)
            if (!input.includeRaw) delete (normalized as any).raw
            return normalized
        },
    },
    panel: {
        async get(input: {characterId: number}) { return deepCopy(character(input.characterId).getPanel()) },
    },
    optimize: {
        async run(input: {characterId: number, applyResult?: number | false}) {
            try {
                const result = await character(input.characterId).optimize(input)
                emitMonaApiEvent("calculator.optimization.completed", {characterId: input.characterId})
                return deepCopy(result)
            } catch (error) {
                throwComputationError(error)
            }
        },
    },
    bestArtifactSet: {
        async run(input: {presetName?: string, config?: any, timeout?: number}) {
            let config = input.config
            if (!config && input.presetName) {
                const item = requireValue(usePresetStore().getPreset(input.presetName), "NOT_FOUND", `Preset ${input.presetName} does not exist`).item
                config = {
                    character: item.character,
                    weapon: item.weapon,
                    target_function: item.targetFunction,
                    buffs: item.buffs,
                    enemy: null,
                    artifact_config: item.artifactEffectMode === "custom" ? item.artifactConfig : null,
                }
            }
            if (!config) throw new MonaApiError("INVALID_ARGUMENT", "presetName or config is required")
            try {
                const results: any = await wasmCalcBestArtifactSet(config, input.timeout ?? 120000)
                const sorted = [...results].sort((a: any, b: any) => b.value - a.value)
                const maxValue = sorted[0]?.value ?? 0
                return sorted.map((item: any) => ({
                    stats: item.stats,
                    value: item.value,
                    setType: item.set_type,
                    ratio: maxValue === 0 ? 0 : item.value / maxValue,
                }))
            } catch (error) {
                throwComputationError(error)
            }
        },
    },
}
