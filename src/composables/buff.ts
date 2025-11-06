// @ts-ignore
import {buffData} from "@buff"
import {RandomIDProvider} from "@/utils/idProvider"
import type {IBuffWasm} from "@/types/preset"
import { config } from "localforage"

export interface BuffEntry {
    id: number,
    name: string,
    config: any,
    configValue: any,
    configUnlinked: any,
    lock: boolean,
}

export function useBuff() {
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
                config: buff.configValue,
                originalConfig: buff.config,
                configUnlinked: buff.configUnlinked,
            })
        }
        return temp
    })

    function addBuff(name: string) {
        const data = buffData[name]
        let defaultConfig: any = {}
        for (let c of data.config) {
            defaultConfig[c.name] = c.default
        }

        let config
        if (data.config.length === 0) {
            config = "NoConfig"
        } else {
            config = {
                [name]: defaultConfig
            }
        }

        let defaultUnlinked: any = {}
        for (let c of data.config) {
            defaultUnlinked[c.name] = c.unlinked
        }

        let configUnlinked
        if (data.config.length === 0) {
            configUnlinked = {}
        } else {
            configUnlinked = {
                [name]: defaultUnlinked
            }
        }
        
        buffs.value.push({
            name,
            config,
            configValue: structuredClone(config),
            configUnlinked,
            id: idGenerator.generateId(),
            lock: false
        })
    }

    function deleteBuff(id: number) {
        const index = buffs.value.findIndex(e => e.id === id)
        console.log(0)
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

