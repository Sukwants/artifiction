import {useArtifactStore} from "@/store/pinia/artifact"
import type {ArtifactSetName, IArtifact, IArtifactWasm} from "@/types/artifact"
// @ts-ignore
import {artifactsData} from "@artifact"
import {convertArtifact, convertArtifactName} from "@/utils/converter"
import {toSnakeCase} from "@/utils/common"
import {getArtifactAllConfigsByName, newDefaultArtifactConfig} from "@/utils/artifacts"
import {useI18n} from "@/i18n/i18n"
import { get } from "lodash"
import { getObjectConfigValue } from "./globalConfig"

export function useArtifactConfig() {

    const artifactConfig = ref<any>(newDefaultArtifactConfig())
    const artifactConfigValue = computed(() => {
        return getObjectConfigValue(artifactConfig.value)
    })
    const showConfigArtifactDialog = ref(false)
    const artifactEffectMode = ref<"auto" | "custom">("custom")

    function handleClickArtifactConfig() {
        showConfigArtifactDialog.value = true
    }

    return {
        artifactConfig,
        artifactConfigValue,
        showConfigArtifactDialog,
        artifactEffectMode,
        handleClickArtifactConfig,
    }
}

export function use5Artifacts() {
    const artifactStore = useArtifactStore()

    const artifactIds = ref([-1, -1, -1, -1, -1])
    // artifact set 2/4 config
    const artifactSingleConfig = ref<any>(newDefaultArtifactConfig())

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
        return getObjectConfigValue(artifactSingleConfig.value)
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

        setArtifact,
        removeArtifact,
        toggleArtifact,
    }
}
