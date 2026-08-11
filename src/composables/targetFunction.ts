import type {TargetFunctionName} from "@/types/targetFunction"
// @ts-ignore
import {targetFunctionByCharacterName, targetFunctionData} from "@targetFunction"
import {type Ref} from "vue"
import type {CharacterName} from "@/types/character"
import {useI18n} from "@/i18n/i18n";
import { ConfigMeta, ConfigAddress, ConfigManager } from "@/composables/config"

function getDefaultTargetFunction(name: string) {
    let res: any;

    const characterTfList = targetFunctionByCharacterName[name]
    if (characterTfList && characterTfList.length > 0) {
        res = characterTfList[0].name
    } else {
        res = targetFunctionByCharacterName["common"][0].name
    }

    return res;
}

// export function getDefaultTargetFunctionConfig(name: string) {
//     let res: any;

//     const hasConfig = targetFunctionData[name].config.length > 0

//     if (hasConfig) {
//         let defaultConfig: any = {}
//         for (let c of targetFunctionData[name].config) {
//             defaultConfig[c.name] = {
//                 config: c.default,
//                 unlinked: c.unlinked,
//             }
//         }
//         res = {
//             [name]: defaultConfig
//         }
//     } else {
//         res = "NoConfig"
//     }

//     return res;
// }

export function useTargetFunction(characterName: Ref<CharacterName>, config: ConfigManager, character_id: number) {
    const targetFunctionName = ref<TargetFunctionName>(getDefaultTargetFunction(characterName.value))
    const targetFunctionConfig = ref<Record<string, Record<string, ConfigAddress>>>({
        [targetFunctionName.value]: config.registerObject(character_id, "target_function", targetFunctionName.value, targetFunctionData[targetFunctionName.value].config)
    })
    const targetFunctionUseDSL = ref(false)
    const targetFunctionDSLSource = ref("")

    const { t, ta } = useI18n()

    const targetFunctionBadge = computed(() => {
        return targetFunctionData[targetFunctionName.value].badge
    })

    const targetFunctionDescription = computed(() => {
        // return targetFunctionData[targetFunctionName.value].description
        return ta(targetFunctionData[targetFunctionName.value].description)
    })

    const targetFunctionNeedConfig = computed(() => {
        const temp = targetFunctionData[targetFunctionName.value].config
        return temp && temp.length > 0
    })

    const targetFunctionConfigMeta: Ref<ConfigMeta[]> = computed(() => {
        return targetFunctionData[targetFunctionName.value].config
    })

    const targetFunctionInterface = computed(() => {
        const use_dsl = targetFunctionUseDSL.value
        return {
            name: targetFunctionName.value,
            params: config.getModuleValue(targetFunctionConfig.value),
            use_dsl,
            dsl_source: use_dsl ? targetFunctionDSLSource.value : ""
        }
    })

    watch(() => characterName.value, name => {
        targetFunctionName.value = getDefaultTargetFunction(name)
    }, {
        flush: "sync"
    })

    watch(() => targetFunctionName.value, (name, oldName) => {
        if (oldName && targetFunctionConfig.value[oldName]) {
            config.unregisterObject(targetFunctionConfig.value[oldName])
        }
        targetFunctionConfig.value = {
            [name]: config.registerObject(character_id, "target_function", name, targetFunctionData[name].config)
        }
    }, {
        flush: "sync"
    })

    return {
        targetFunctionName,
        targetFunctionConfig,
        targetFunctionUseDSL,
        targetFunctionDSLSource,
        targetFunctionBadge,
        targetFunctionDescription,
        targetFunctionNeedConfig,
        targetFunctionConfigMeta,
        targetFunctionConfigConfig: targetFunctionConfigMeta,
        targetFunctionInterface
    }
}

