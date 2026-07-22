import {upgradePresetItem} from "@/utils/preset"
import {computed, reactive, type Ref, ref, watch} from "vue"
import {type IPreset} from "@/types/preset"

const VERSION = 4

function loadPresetOrDefault(payload: any) {
    if (!payload) {
        return {}
    } else {
        for (let name in payload) {
            let entry = payload[name]
            let item = entry.item

            try {
                entry.item = upgradePresetItem(item)
            } catch (e) {
                console.log("upgrade preset item failed")
                console.log(e)
            }
        }

        return payload
    }
}

export interface PresetEntry {
    name: string,
    item: IPreset,
    version: number
}

function f() {
    const presets: Ref<Record<string, PresetEntry>> = ref(loadPresetOrDefault(null))

    function init(payload: any) {
        presets.value = loadPresetOrDefault(payload)
    }

    function addOrOverwrite(name: string, item: IPreset, version: number = VERSION) {
        const preset = upgradePresetToNewVersion({
            name: name,
            item: item,
            version: version
        })
        if (preset) {
            presets.value[name] = preset
        }
    }

    function getPreset(name: string): PresetEntry | undefined {
        return presets.value[name]
    }

    function deletePreset(name: string) {
        delete presets.value[name]
    }

    const allFlat = computed(() => {
        return Object.values(presets.value)
    })

    const count = computed(() => {
        return Object.keys(presets.value).length;
    })

    return {
        presets,

        init,
        addOrOverwrite,
        deletePreset,
        getPreset,

        allFlat,
        count
    }
}

const s = f()

export function watchContent() {
    return s.presets.value
}

// watch(() => {
//     return s.presets.value
// }, newValue => {
//     localStorage.setItem("presets5", JSON.stringify(newValue))
// }, {
//     deep: true
// })

export const usePresetStore = () => {
    return s
}

export function upgradePresetToNewVersion(preset: PresetEntry | any): PresetEntry | undefined {
    if (preset.version === 4) {
        return preset;
    }
    if (preset.version <= 3) {
        const item = preset.item as any;

        function removeConfigUnlinked(obj: any) {
            const {configUnlinked, ...rest} = obj;
            return rest;
        }
        
        let res: IPreset = {
            name: item.name,
            algorithm: item.algorithm,
            artifactConfig: item.artifactConfig,
            artifactEffectMode: item.artifactEffectMode,
            constraint: item.constraint,
            dslSource: item.dslSource,
            useDSL: item.useDSL,
            filter: item.filter,
            character: removeConfigUnlinked(item.character),
            weapon: removeConfigUnlinked(item.weapon),
            targetFunction: removeConfigUnlinked(item.targetFunction),
            buffs: item.buffs.map((buff: any) => removeConfigUnlinked(buff)),
            globalConfigUnlinked: {},   // 不管了，不要了
        }

        if (res.character.params == "NoConfig") res.character.params = {};
        if (res.weapon.params == "NoConfig") res.weapon.params = {};
        if (res.targetFunction.params == "NoConfig") res.targetFunction.params = {};
        if (res.buffs) {
            for (const id in res.buffs) {
                if (res.buffs[id].config == "NoConfig") res.buffs[id].config = {};
            }
        }
        if (res.artifactConfig == "NoConfig") res.artifactConfig = {};

        return {
            name: preset.name,
            item: res,
            version: 4
        }
    }
    return undefined;
}