import type {TargetFunctionName} from "@/types/targetFunction"
// @ts-ignore
import {targetFunctionByCharacterName, targetFunctionData} from "@targetFunction"
import {type Ref} from "vue"
import type {CharacterName} from "@/types/character"
import {useI18n} from "@/i18n/i18n";

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

function getDefaultTargetFunctionConfig(name: string) {
    let res: any;

    const hasConfig = targetFunctionData[name].config.length > 0

    if (hasConfig) {
        let defaultConfig: any = {}
        for (let c of targetFunctionData[name].config) {
            defaultConfig[c.name] = c.default
        }
        res = {
            [name]: defaultConfig
        }
    } else {
        res = "NoConfig"
    }

    return res;
}

function getDefaultTargetFunctionUnlinked(name: string) {
    let res: any = {};

    if (!!targetFunctionData[name]?.config) {
        let defaultConfigUnlinked: any = {}
        for (let c of targetFunctionData[name].config) {
            defaultConfigUnlinked[c.name] = c.unlinked
        }
        res = {
            [name]: defaultConfigUnlinked
        }
    }

    return res;
}

export function useTargetFunction(characterName: Ref<CharacterName>) {
    const targetFunctionName = ref<TargetFunctionName>(getDefaultTargetFunction(characterName.value))
    const targetFunctionConfig = ref<any>("NoConfig")
    const targetFunctionConfigValue = ref<any>("NoConfig")
    const targetFunctionConfigUnlinked = ref<any>({})
    const targetFunctionUseDSL = ref(false)
    const targetFunctionDSLSource = ref("")

    targetFunctionConfig.value = getDefaultTargetFunctionConfig(getDefaultTargetFunction(characterName.value))
    targetFunctionConfigValue.value = getDefaultTargetFunctionConfig(getDefaultTargetFunction(characterName.value))
    targetFunctionConfigUnlinked.value = getDefaultTargetFunctionUnlinked(getDefaultTargetFunction(characterName.value))

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

    const targetFunctionConfigConfig = computed(() => {
        targetFunctionConfigUnlinked.value = getDefaultTargetFunctionUnlinked(getDefaultTargetFunction(characterName.value))
        return targetFunctionData[targetFunctionName.value].config
    })

    const targetFunctionInterface = computed(() => {
        const use_dsl = targetFunctionUseDSL.value
        return {
            name: targetFunctionName.value,
            params: targetFunctionConfigValue.value,
            configUnlinked: targetFunctionConfigUnlinked.value,
            use_dsl,
            dsl_source: use_dsl ? targetFunctionDSLSource.value : ""
        }
    })

    watch(() => characterName.value, name => {
        targetFunctionName.value = getDefaultTargetFunction(name)
    }, {
        flush: "sync"
    })

    watch(() => targetFunctionName.value, name => {
        targetFunctionName.value = name
        targetFunctionConfig.value = getDefaultTargetFunctionConfig(name)
    }, {
        flush: "sync"
    })

    return {
        targetFunctionName,
        targetFunctionConfig,
        targetFunctionConfigValue,
        targetFunctionConfigUnlinked,
        targetFunctionUseDSL,
        targetFunctionDSLSource,
        targetFunctionBadge,
        targetFunctionDescription,
        targetFunctionNeedConfig,
        targetFunctionConfigConfig,
        targetFunctionInterface
    }
}

