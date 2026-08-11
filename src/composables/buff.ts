// @ts-ignore
import {buffData} from "@buff"
import {RandomIDProvider} from "@/utils/idProvider"
import type {IBuffWasm} from "@/types/preset"
import { ConfigAddress, ConfigManager } from "@/composables/config"

export interface BuffEntry {
    id: number,
    name: string,
    config: Record<string, Record<string, ConfigAddress>>,
    lock: boolean,
}

// export function getDefaultBuffConfig(name: string) {
//     const data = buffData[name]

//     let defaultConfig: any = {}
//     for (let c of data.config) {
//         defaultConfig[c.name] = structuredClone(c.default)
//     }

//     return {
//         [name]: defaultConfig
//     }
// }

export function useBuff(config: ConfigManager, character_id: number) {
    const buffs = ref<BuffEntry[]>([])

    const idGenerator = new RandomIDProvider()

    const buffsUnlocked = computed((): BuffEntry[] => {
        return buffs.value.filter(e => !e.lock)
    })

    const buffsInterface = computed((): IBuffWasm[] => {
        let temp = []
        for (let buff of buffsUnlocked.value) {
            temp.push({
                name: buff.name,
                config: config.getModuleValue(buff.config),
            })
        }
        return temp
    })

    function addBuff(name: string) {
        const id = idGenerator.generateId()
        buffs.value.push({
            name,
            config: {
                [name]: config.registerObject(character_id, "buff", `${name}-${id}`, buffData[name].config),
            },
            id: id,
            lock: false
        })
    }

    function deleteBuff(id: number) {
        const index = buffs.value.findIndex(e => e.id === id)
        config.unregisterObject(buffs.value[index].config[buffs.value[index].name])
        buffs.value.splice(index, 1)
    }

    function toggleBuff(id: number) {
        const index = buffs.value.findIndex(e => e.id === id)
        const v = buffs.value[index].lock
        buffs.value[index].lock = !v
    }

    return {
        buffs,

        buffsInterface,

        addBuff,
        deleteBuff,
        toggleBuff,
    }
}

