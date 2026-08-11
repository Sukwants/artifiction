const DEFAULT_WEAPON = "ATeaspoonOfTranscendence";

import type {WeaponName, WeaponType} from "@/types/weapon"
// @ts-ignore
import {weaponByType, weaponData} from "@weapon"
import {type Ref} from "vue"
import {useI18n} from "@/i18n/i18n";
import { ConfigMeta, ConfigAddress, ConfigManager } from "@/composables/config"

// export function getDefaultWeaponConfig(name: string) {
//     let res: any;

//     // change config
//     const hasConfig = !!weaponData[name]?.configs
//     if (hasConfig) {
//         const configs = weaponData[name].configs

//         let defaultConfig: any = {}
//         for (let config of configs) {
//             defaultConfig[config.name] = {
//                 config: config.default,
//                 unlinked: config.unlinked,
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
export function useWeapon(weaponType: null | Ref<WeaponType>, config: ConfigManager, character_id: number) {
    const weaponName = ref(DEFAULT_WEAPON)
    const weaponLevel = ref("90")
    const weaponRefine = ref(1)
    const weaponConfig = ref<Record<string, Record<string, ConfigAddress>>>({
        [weaponName.value]: config.registerObject(character_id, "weapon", weaponName.value, weaponData[weaponName.value].configs)
    })

    const weaponLevelNumber = computed(() => {
        return parseInt(weaponLevel.value)
    })

    const weaponAscend = computed(() => {
        return weaponLevel.value.includes("+")
    })

    const weaponSplash = computed(() => {
        const data = weaponData[weaponName.value]
        return data.gacha ?? data.url ?? data.tn
    })

    const weaponNeedConfig = computed(() => {
        return !!weaponData[weaponName.value].configs
    })

    const weaponConfigMeta: Ref<ConfigMeta[]> = computed(() => {
        return weaponData[weaponName.value].configs
    })

    const weaponInterface = computed(() => {
        return {
            name: weaponName.value,
            level: weaponLevelNumber.value,
            ascend: weaponAscend.value,
            refine: weaponRefine.value,
            params: config.getModuleValue(weaponConfig.value),
        }
    })

    // function changeWeapon(name: WeaponName) {
    //
    // }

    if (weaponType) {
        watch(() => weaponType.value, newWeaponType => {
            const defaultWeaponData = weaponByType[newWeaponType][0]
            weaponName.value = defaultWeaponData.name
        }, {
            flush: "sync"
        })
    }

    watch(() => weaponName.value, (name, oldName) => {
        if (oldName && weaponConfig.value[oldName]) {
            config.unregisterObject(weaponConfig.value[oldName])
        }
        weaponConfig.value = {
            [name]: config.registerObject(character_id, "weapon", name, weaponData[name].configs)
        }
    }, {
        flush: "sync"
    })

    const { ta } = useI18n()

    const weaponLocale = computed(() => {
        return ta(weaponData[weaponName.value].nameLocale)
    })

    return {
        weaponName,
        weaponLevel,
        weaponRefine,
        weaponConfig,
        weaponLevelNumber,
        weaponAscend,
        weaponSplash,
        weaponNeedConfig,
        weaponConfigMeta,
        weaponInterface,
        weaponLocale

        // changeWeapon
    }
}
