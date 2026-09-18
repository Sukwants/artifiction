import {characterData} from "@/assets/character"
import {weaponData, weaponByType} from "@/assets/weapon"
import {targetFunctionData, targetFunctionByCharacterName} from "@/assets/target_function"
import {buffData} from "@/assets/buff"
import {artifactsData} from "@/assets/artifacts"
import {deepCopy} from "@/utils/common"

function describe(name: string, data: any) {
    return {
        key: name,
        nameLocale: data.nameLocale,
        configSchema: deepCopy(data.config ?? data.configs ?? []),
    }
}

export const metaApi = {
    characters: {
        async list() {
            return Object.entries(characterData).map(([name, data]: [string, any]) => ({
                ...describe(name, data), element: data.element, weaponType: data.weapon, star: data.star,
            }))
        },
        async get(name: string) { return deepCopy((characterData as any)[name]) },
    },
    weapons: {
        async list(query: {characterName?: string, weaponType?: string} = {}) {
            const type = query.weaponType ?? (query.characterName ? (characterData as any)[query.characterName]?.weapon : undefined)
            const entries: any[] = type ? (weaponByType as any)[type] ?? [] : Object.values(weaponData)
            return entries.map(data => ({...describe(data.name, data), type: data.type, star: data.star}))
        },
        async get(name: string) { return deepCopy((weaponData as any)[name]) },
    },
    targetFunctions: {
        async list(query: {characterName?: string} = {}) {
            const entries = query.characterName
                ? [...((targetFunctionByCharacterName as any)[query.characterName] ?? []), ...((targetFunctionByCharacterName as any).common ?? [])]
                : Object.values(targetFunctionData)
            return entries.map((data: any) => ({...describe(data.name, data), for: data.for}))
        },
        async get(name: string) { return deepCopy((targetFunctionData as any)[name]) },
    },
    buffs: {
        async list() { return Object.entries(buffData).map(([name, data]: [string, any]) => ({...describe(name, data), genre: data.genre})) },
        async get(name: string) { return deepCopy((buffData as any)[name]) },
    },
    artifactSets: {
        async list() { return Object.entries(artifactsData).map(([name, data]: [string, any]) => ({...describe(name, data), configs: [1, 2, 3, 4, 5].map(n => deepCopy(data[`config${n}`] ?? []))})) },
        async get(name: string) { return deepCopy((artifactsData as any)[name]) },
    },
    skills: {
        async list(query: {characterName: string}) {
            const data = (characterData as any)[query.characterName]
            if (!data) return []
            return [data.skillMap1, data.skillMap2, data.skillMap3].flat().map((skill: any) => ({index: skill.index, nameLocale: skill.text}))
        },
    },
    reactions: {
        async list() {
            return {
                transformative: ["swirl_cryo", "swirl_pyro", "swirl_hydro", "swirl_electro", "overload", "electro_charged", "shatter", "superconduct", "bloom", "hyperbloom", "burgeon", "burning", "crystallize"],
                elevative: ["lunar_charged_reaction", "lunar_crystallize_reaction", "stellar_swirl_reaction_anemo", "stellar_swirl_reaction_cryo"],
            }
        },
    },
    infusions: {
        async list() { return ["None", "Pyro", "Electro", "Hydro", "Anemo", "Geo", "Cryo", "Dendro"] },
    },
}
