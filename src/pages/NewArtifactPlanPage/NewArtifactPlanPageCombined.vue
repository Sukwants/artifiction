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
                :teamSharedGlobalConfig="teamSharedGlobalConfig"
                @update:interface="val => characters[id] = val"
                @update:configList="val => configList[id] = val"
            ></new-artifact-plan-page>
        </el-tab-pane>
    </el-tabs>
</template>

<script setup lang="ts">
import NewArtifactPlanPage from "./NewArtifactPlanPage.vue"
import type { TabPaneName } from 'element-plus'
import {useI18n} from "@/i18n/i18n"
import { characterData } from "@/assets/character";
import { processSharedGlobalConfig } from "@/composables/globalConfig";

const { t, ta } = useI18n()

let currentCharacterId = ref(1)
let characterIds = ref([1])
let maxCharacterId = 1

const editCharacter = (
  targetName: TabPaneName | undefined,
  action: 'remove' | 'add'
) => {
    if (action === 'add') {
        maxCharacterId++
        characterIds.value.push(maxCharacterId)
        currentCharacterId.value = maxCharacterId

        characterTeamIds.value[maxCharacterId] = 1
        characterOnField.value[maxCharacterId] = false
    } else if (action === 'remove') {
        const index = characterIds.value.findIndex((id) => id === targetName)
        if (index !== -1) {
            characters.value[characterIds.value[index]] = null
            configList.value[characterIds.value[index]] = null
            if (currentCharacterId.value === targetName) {
                currentCharacterId.value = characterIds.value[index + 1] || characterIds.value[index - 1] || 0
            }
            characterIds.value.splice(index, 1)
        }
    }
}

let characterTeamIds = ref<number[]>([0, 1])
let maxTeamId = computed(() => {
    return Math.max(...characterTeamIds.value, 1)
})
let characterOnField = ref<boolean[]>([false, true])


const characters = ref<any[]>([])
const configList = ref<any[]>([])

const get_character_name = (id: number) => {
    return (characterData as any)[characters.value[id]?.character?.name]?.nameLocale
}

let teamSharedGlobalConfig = computed(() => {
    let values: any = {};
    for (let i of configList.value) {
        for (let key in i) {
            if (!values[key]) values[key] = [];
            values[key].push(...i[key])
        }
    }
    return values
})


</script>

<style scoped>

</style>