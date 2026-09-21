<template>
    <el-tabs
        v-model="currentCharacterId"
        type="card"
        editable
        @edit="editCharacter"
    >
        <el-tab-pane
            v-for="id in characterIds"
            :key="id"
            :name="id"
        >
            <template #label>
                <span>
                    {{ ta(get_character_name(id)) }}
                </span>
                <el-select
                    v-model="characterTeamIds[id]"
                    size="small"
                    style="width: 80px; margin-left: 8px;"
                >
                    <template #header>
                        <el-checkbox
                            v-model="characterOnField[id]"
                            size="small"
                        >
                            {{ t("calcPage.onField") }}
                        </el-checkbox>
                    </template>
                    <template #prefix>
                        <span v-if="characterOnField[id]" style="color: var(--el-color-primary);"> Team </span>
                        <span v-else style="color: var(--el-text-color-disabled);"> Team </span>
                    </template>
                    <el-option
                        v-for="teamId in maxTeamId + 1"
                        :key="teamId"
                        :value="teamId"
                    />
                </el-select>
            </template>
            <new-artifact-plan-page
                :characters="characters"
                :currentCharacterId="id"
                :currentTeamId="characterTeamIds[id]"
                :currentOnField="characterOnField[id]"
                @register-controller="controller => setCharacterController(id, controller)"
                @update:interface="val => characters[id] = val"
            ></new-artifact-plan-page>
        </el-tab-pane>
    </el-tabs>
</template>

<script setup lang="ts">
import NewArtifactPlanPage from "./NewArtifactPlanPage.vue"
import type { TabPaneName } from 'element-plus'
import {useI18n} from "@/i18n/i18n"
import { characterData } from "@/assets/character";
import { ref, computed } from 'vue';
import { ConfigManager } from "@/composables/config";
import {registerCalculatorController, type CalculatorCharacterController} from "@/api/mona/calculatorRuntime";
import {MonaApiError} from "@/api/mona/errors";
import {deepCopy} from "@/utils/common";

const { t, ta } = useI18n()

let currentCharacterId = ref(1)
let characterIds = ref([1])
let maxCharacterId = 1

const configManager = new ConfigManager()
const characterControllers = new Map<number, CalculatorCharacterController>()

provide("configManager", configManager)

function setCharacterController(id: number, controller: any) {
    if (!controller) {
        characterControllers.delete(id)
        return
    }
    characterControllers.set(id, controller as CalculatorCharacterController)
}

async function waitForCharacterController(id: number) {
    for (let i = 0; i < 100; i++) {
        const controller = characterControllers.get(id)
        if (controller) return controller
        await new Promise(resolve => setTimeout(resolve, 10))
    }
    throw new MonaApiError("PAGE_NOT_READY", `Calculator character ${id} is not ready`)
}

async function addCharacter(input?: any) {
    maxCharacterId++
    const id = maxCharacterId
    characterIds.value.push(id)
    characterTeamIds.value[id] = input?.teamId ?? 1
    characterOnField.value[id] = input?.onField ?? false
    currentCharacterId.value = id
    await nextTick()
    const controller = await waitForCharacterController(id)
    if (input) await controller.patchState(input)
    return controller.getState()
}

async function removeCharacter(id: number) {
    const index = characterIds.value.indexOf(id)
    if (index < 0) throw new MonaApiError("NOT_FOUND", `Calculator character ${id} does not exist`)
    characters.value[id] = null
    configManager.removeCharacter(id)
    characterControllers.delete(id)
    if (currentCharacterId.value === id) {
        currentCharacterId.value = characterIds.value[index + 1] || characterIds.value[index - 1] || 0
    }
    characterIds.value.splice(index, 1)
    await nextTick()
}

const editCharacter = (
  targetName: TabPaneName | undefined,
  action: 'remove' | 'add'
) => {
    if (action === 'add') {
        void addCharacter()
    } else if (action === 'remove') {
        if (targetName !== undefined) void removeCharacter(Number(targetName))
    }
}

let characterTeamIds = ref<number[]>([0, 1])
let maxTeamId = computed(() => {
    return Math.max(...characterTeamIds.value, 1)
})
let characterOnField = ref<boolean[]>([false, true])


const characters = ref<any[]>([])

const get_character_name = (id: number) => {
    return (characterData as any)[characters.value[id]?.character?.name]?.nameLocale
}

const pageController = {
    listCharacters() {
        return characterIds.value.map(id => ({
            id,
            name: characters.value[id]?.character?.name,
            teamId: characterTeamIds.value[id],
            onField: characterOnField.value[id],
            selected: currentCharacterId.value === id,
        }))
    },
    getCharacter(id: number) {
        return characterControllers.get(id)
    },
    addCharacter,
    removeCharacter,
    getSelectedCharacterId() {
        return currentCharacterId.value || null
    },
    async selectCharacter(id: number) {
        if (!characterIds.value.includes(id)) throw new MonaApiError("NOT_FOUND", `Calculator character ${id} does not exist`)
        currentCharacterId.value = id
        await nextTick()
    },
    async patchCharacter(id: number, patch: any) {
        if (!characterIds.value.includes(id)) throw new MonaApiError("NOT_FOUND", `Calculator character ${id} does not exist`)
        if (patch.teamId !== undefined) characterTeamIds.value[id] = patch.teamId
        if (patch.onField !== undefined) characterOnField.value[id] = patch.onField
        const controller = await waitForCharacterController(id)
        return controller.patchState(patch)
    },
    getState() {
        return {
            schema: "mona.calculator-state",
            schemaVersion: 1,
            selectedCharacterId: currentCharacterId.value || null,
            characters: characterIds.value.map(id => characterControllers.get(id)?.getState()).filter(Boolean),
        }
    },
    async setState(state: any, mode: "replace" | "merge" = "replace") {
        if (!state || !Array.isArray(state.characters)) throw new MonaApiError("INVALID_ARGUMENT", "Calculator state must contain characters")
        const source = deepCopy(state.characters)
        if (mode === "replace") {
            while (characterIds.value.length > source.length) await removeCharacter(characterIds.value[characterIds.value.length - 1])
        }
        const idMap = new Map<number, number>()
        for (let index = 0; index < source.length; index++) {
            const item = source[index]
            let id = item.id
            if (!characterIds.value.includes(id)) {
                if (mode === "replace" && index < characterIds.value.length) id = characterIds.value[index]
                else id = (await addCharacter()).id
            }
            idMap.set(item.id, id)
            await pageController.patchCharacter(id, item)
        }
        const selected = idMap.get(state.selectedCharacterId) ?? state.selectedCharacterId
        if (selected && characterIds.value.includes(selected)) await pageController.selectCharacter(selected)
        return pageController.getState()
    },
}

let unregisterController: null | (() => void) = null
onActivated(() => { unregisterController = registerCalculatorController(pageController) })
onDeactivated(() => { unregisterController?.(); unregisterController = null })
onMounted(() => { unregisterController = registerCalculatorController(pageController) })
onUnmounted(() => { unregisterController?.(); unregisterController = null })

</script>

<style scoped>

</style>
