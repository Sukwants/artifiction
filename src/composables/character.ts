const DEFAULT_CHARACTER = "Sandrone";

import type { WeaponType } from "@/types/weapon"
// @ts-ignore
import { characterData } from "@character"
import { type Ref } from "vue"
import type { CharacterName } from "@/types/character"
import { useI18n } from "@/i18n/i18n"
import { ConfigMeta, ConfigAddress, ConfigManager } from "@/composables/config"

// export function getDefaultCharacterConfig(name: string) {
//     let res: any = {};

//     const hasConfigData = characterData[name].config.length > 0;

//     // change config
//     if (hasConfigData) {
//         const configs = characterData[name].config

//         let defaultConfig: any = {}
//         for (let c of configs) {
//             defaultConfig[c.name] = structuredClone(c.default)
//         }
//         res = {
//             [name]: defaultConfig
//         }
//     }

//     return res;
// }

export function getDefaultCharacterTag(name: string) {
    return characterData[name].defaultTags || []
}

export function useCharacter(config: ConfigManager, character_id: number) {
    const characterName = ref(DEFAULT_CHARACTER)
    const characterLevel = ref("90")
    const characterConfig = ref<Record<string, Record<string, ConfigAddress>>>({
        [characterName.value]: config.registerObject(character_id, "character", characterName.value, characterData[characterName.value].config)
    })
    const characterSkill1 = ref(8)
    const characterSkill2 = ref(8)
    const characterSkill3 = ref(8)
    const characterConstellation = ref(0)

    const { ta } = useI18n()
    // const characterWeaponType = ref<WeaponType>("Bow")

    const characterWeaponType = computed(() => {
        const data = characterData[characterName.value]
        return data.weapon
    })

    const characterLevelNumber = computed((): number => {
        return parseInt(characterLevel.value)
    })

    const characterAscend = computed((): boolean => {
        return characterLevel.value.includes("+")
    })

    const characterSplash = computed((): string => {
        const data = characterData[characterName.value]
        return data.splash
    })

    const characterNeedConfig = computed(() => {
        let temp = characterData[characterName.value].config
        return temp && temp.length > 0
    })

    const characterConfigMeta: Ref<ConfigMeta[]> = computed(() => {
        return characterData[characterName.value].config
    })

    const characterLocale = computed(() => {
        return ta(characterData[characterName.value].nameLocale)
    })

    const characterTags = ref(getDefaultCharacterTag(characterName.value))

    const characterInterface = computed(() => {
        let i = {
            name: characterName.value,
            level: characterLevelNumber.value,
            ascend: characterAscend.value,
            constellation: characterConstellation.value,
            skill1: characterSkill1.value - 1,
            skill2: characterSkill2.value - 1,
            skill3: characterSkill3.value - 1,
            params: config.getModuleValue(characterConfig.value),
            tags: characterTags.value,
        }
        return i
    })

    watch(() => characterName.value, name => {
        characterName.value = name
        config.unregisterObject(characterConfig.value[characterName.value])
        characterConfig.value = {
            [name]: config.registerObject(character_id, "character", name, characterData[name].config)
        } 
        characterTags.value = getDefaultCharacterTag(name)
    }, {
        flush: "sync"
    })

    return {
        characterName,
        characterLevel,
        characterConfig,
        characterSkill1,
        characterSkill2,
        characterSkill3,
        characterConstellation,
        characterWeaponType,
        characterLevelNumber,
        characterAscend,
        characterSplash,
        characterNeedConfig,
        characterConfigMeta,
        characterInterface,
        characterLocale,
        characterTags,
    }
}

// function getDefaultCharacterSkillConfig(name: string) {
//     let res: any;

//     const hasConfigSkill = characterData[name].configSkill.length > 0

//     // change skill config
//     if (hasConfigSkill) {
//         let defaultConfig: any = {}
//         for (let c of characterData[name].configSkill) {
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

export function useCharacterSkill(characterName: Ref<CharacterName>, config: ConfigManager, character_id: number) {
    const characterSkillConfig = ref<Record<string, Record<string, ConfigAddress>>>({
        [characterName.value]: config.registerObject(character_id, "character_skill", characterName.value, characterData[characterName.value].configSkill)
    })
    const characterSkillIndex = ref(0)

    const characterNeedSkillConfig = computed((): boolean => {
        let temp = characterData[characterName.value].configSkill
        return temp && temp.length > 0
    })

    const characterSkillConfigMeta = computed(() => {
        return characterData[characterName.value].configSkill
    })

    const characterSkillInterface = computed(() => {
        return {
            index: characterSkillIndex.value,
            config: config.getModuleValue(characterSkillConfig.value)
        }
    })

    watch(() => characterName.value, name => {
        config.unregisterObject(characterSkillConfig.value[name])
        characterSkillConfig.value = {
            [name]: config.registerObject(character_id, "character_skill", name, characterData[name].configSkill)
        }
        // change skill index
        characterSkillIndex.value = 0
    }, {
        flush: "sync"
    })

    // watch(() => characterSkillConfig.value, () => {
    //     console.log("skill config")
    // })

    // watch(() => characterSkillIndex.value, v => {
    //     console.log(v)
    // })

    return {
        characterSkillConfig,
        characterSkillIndex,
        characterNeedSkillConfig,
        characterSkillConfigMeta,
        characterSkillInterface,
    }
}
