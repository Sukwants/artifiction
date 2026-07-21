import {useArtifactStore} from "@/store/pinia/artifact"
import type {ArtifactSetName, IArtifact, IArtifactWasm} from "@/types/artifact"
// @ts-ignore
import {artifactsData} from "@artifact"
import {convertArtifact, convertArtifactName} from "@/utils/converter"
import {toSnakeCase} from "@/utils/common"
import { default_artifact_config } from "@/utils/artifacts"
import {useI18n} from "@/i18n/i18n"
import { ConfigAddress, ConfigManager } from "@/composables/config"

export function useArtifactConfig(config: ConfigManager, character_id: number) {

    const artifactConfig = ref<Record<string, Record<string, ConfigAddress>>>({})
    for (const name in artifactsData) {
        const data = artifactsData[name]
        const name2 = data.name2

        const snake = toSnakeCase(name2)

        for (const num of [1, 2, 3, 4, 5]) {
            artifactConfig.value[`config_${snake}*set${num}`] = config.registerObject(character_id, "artifact_config", `${name}*set${num}`, data[`config${num}`])
        }
    }
    const showConfigArtifactDialog = ref(false)
    const artifactEffectMode = ref<"auto" | "custom">("custom")

    function handleClickArtifactConfig() {
        showConfigArtifactDialog.value = true
    }

    return {
        artifactConfig,
        showConfigArtifactDialog,
        artifactEffectMode,
        handleClickArtifactConfig,
    }
}

export function use5Artifacts(config: ConfigManager, character_id: number) {
    const artifactStore = useArtifactStore()

    const artifactIds = ref([-1, -1, -1, -1, -1])
    // artifact set 2/4 config
    const artifactSingleConfig = ref<Record<string, Record<string, ConfigAddress>>>({})
    for (const name in artifactsData) {
        const data = artifactsData[name]
        const name2 = data.name2

        const snake = toSnakeCase(name2)

        for (const num of [1, 2, 3, 4, 5]) {
            artifactSingleConfig.value[`config_${snake}*set${num}`] = {};
        }
    }

    const artifactItems = computed(() => {
        let temp: (IArtifact | null)[] = []
        for (let id of artifactIds.value) {
            if (id >= 0) {
                const a = artifactStore.artifacts.value.get(id)
                if (a) {
                    temp.push(a)
                } else {
                    temp.push(null)
                }
            } else {
                temp.push(null)
            }
        }
        return temp
    })

    const artifactSetCount = computed(() => {
        let temp: Record<ArtifactSetName, number> = {}
        for (let name in artifactsData) {
            temp[name] = 0
        }
        for (let artifact of artifactItems.value) {
            if (!artifact) {
                continue
            }
            const setName = artifact.setName
            if (!Object.prototype.hasOwnProperty.call(temp, setName)) {
                temp[setName] = 0
            }
            temp[setName] += 1
        }
        return temp
    })

    watch(() => artifactSetCount.value, (newVal, oldVal) => {
        for (const name in artifactsData) {
            for (const num of [1, 2, 3, 4, 5]) {
                if (newVal[name] >= num && oldVal[name] < num) {
                    const configName = `config_${toSnakeCase(artifactsData[name].name2)}*set${num}`
                    artifactSingleConfig.value[configName] = config.registerObject(character_id, "artifact_single_config", `${name}*set${num}`, artifactsData[name][`config${num}`])
                }
                if (newVal[name] < num && oldVal[name] >= num) {
                    const configName = `config_${toSnakeCase(artifactsData[name].name2)}*set${num}`
                    config.unregisterObject(artifactSingleConfig.value[configName])
                    artifactSingleConfig.value[configName] = {}
                }
            }
        }
    })

    const artifactWasmFormat = computed((): IArtifactWasm[] => {
        let temp: IArtifactWasm[] = []
        for (let id of artifactIds.value) {
            if (id >= 0) {
                const a = artifactStore.artifacts.value.get(id)
                if (a && !a.omit) {
                    const artifactWasm = convertArtifact(a)
                    temp.push(artifactWasm)
                }
            }
        }
        return temp
    })

    const artifactConfigForCalculator = computed(() => {
        let res: Record<string, Record<string, any>> = {};
        for (const artifact_set_name in artifactsData) {
            const artifact_set_config_name = `config_${toSnakeCase(artifactsData[artifact_set_name].name2)}`
            res[artifact_set_config_name] = {}
            for (const num of [1, 2, 3, 4, 5]) {
                const artifact_set_config_name_with_num = `config_${toSnakeCase(artifactsData[artifact_set_name].name2)}*set${num}`
                const defaultConfig = structuredClone(default_artifact_config[artifact_set_config_name_with_num]) ?? {}
                for (const config_name in defaultConfig) {
                    res[artifact_set_config_name][config_name] = defaultConfig[config_name]
                }
                for (const config_name in artifactSingleConfig.value[artifact_set_config_name_with_num]) {
                    res[artifact_set_config_name][config_name] = config.getConfigValue(artifactSingleConfig.value[artifact_set_config_name_with_num][config_name])
                }
            }
        }
        return res
    })

    const artifactCount = computed(() => {
        let count = 0
        for (const id of artifactIds.value) {
            if (id !== -1) {
                count += 1
            }
        }
        return count
    })

    function setArtifact(index: number, id: number) {
        artifactIds.value[index] = id
    }

    function removeArtifact(index: number) {
        artifactIds.value[index] = -1
    }

    function toggleArtifact(index: number) {
        const a = artifactItems.value[index]
        if (a) {
            const id = a.id
            artifactStore.toggleArtifact(id)
        }
    }

    return {
        artifactIds,
        artifactCount,
        artifactSetCount,
        artifactItems,
        artifactWasmFormat,
        
        artifactSingleConfig,
        artifactConfigForCalculator,
        artifactNeedConfig4: computed(() => false),
        artifactConfig4Configs: computed(() => []),

        setArtifact,
        removeArtifact,
        toggleArtifact,
    }
}
