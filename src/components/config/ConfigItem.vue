<template>
    <div class="config-item">
        <h3 class="config-title">{{ title }}</h3>

        <template v-if="type === 'float'">
            <el-slider
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="0.1"
                :show-input="true"
                :show-input-controls="false"
            />
        </template>

        <template v-else-if="type === 'int'">
            <el-slider
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="1"
            />
        </template>

        <template v-else-if="type === 'intInput'">
            <el-input-number
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
            />
        </template>

        <template v-else-if="type === 'bool'">
            <el-switch
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :active-text="currentLocale.startsWith('zh') ? '是' : 'On'"
            />
        </template>

        <template v-else-if="type === 'floatPercentageInput'">
            <el-input
                :modelValue="modelValue"
                @update:modelValue="handleInputValue"
            >
                <template #append>%</template>
            </el-input>
        </template>

        <template v-else-if="type === 'floatInput'">
            <el-input
                :modelValue="modelValue"
                @update:modelValue="handleInputValue"
            />
        </template>

        <template v-else-if="type === 'element'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="params.elements"
            />
        </template>

        <template v-else-if="type === 'elementOptional'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="[...params.elements, 'None']"
            />
        </template>

        <template v-else-if="type === 'element4'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="['Pyro', 'Cryo', 'Electro', 'Hydro']"
            />
        </template>

        <template v-else-if="type === 'element8'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="['Pyro', 'Cryo', 'Electro', 'Hydro', 'Anemo', 'Geo', 'Dendro', 'Physical']"
            />
        </template>

        <template v-else-if="type === 'elementMulti'">
            <select-element-multi
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="params.elements"
            />
        </template>

        <template v-else-if="type === 'element8multi'">
            <select-element-multi
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
            />
        </template>

        <template v-else-if="type === 'skill4'">
            <select-skill-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
            />
        </template>

        <template v-else-if="type === 'option'">
            <el-radio-group
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
            >
                <el-radio-button
                    v-for="(option, index) in params.options"
                    :key="index"
                    :label="index"
                >{{ option }}</el-radio-button>
            </el-radio-group>
        </template>

        <template v-else-if="type === 'option2'">
            <el-radio-group
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
            >
                <el-radio-button
                    v-for="(option, index) in currentOptions"
                    :key="index"
                    :label="index"
                >{{ option }}</el-radio-button>
            </el-radio-group>
        </template>

        <template v-else-if="type === 'moonsign2'">
            <select-moonsign-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :moonsigns="['Nascent', 'Ascendant']"
            />
        </template>

        <template v-else-if="type === 'moonsign3'">
            <select-moonsign-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :moonsigns="['None', 'Nascent', 'Ascendant']"
            />
        </template>

        <el-tooltip
            v-if="globalLink"
            placement="top"
            :show-after="1000"
        >
            <template #content>
                <div style="position: relative; padding-right: 8px;">
                    <el-link
                        :icon="QuestionFilled"
                        :underline="false"
                        style="position: absolute; top: 0; right: -5px; font-size: 10px;"
                        href="/help/instruction#全局配置"
                    />
                    <div style="display: flex; justify-content: space-between; height: 28px; gap: 16px; align-items: center;">
                        <span><b>Key</b></span>
                        <span>{{ globalLink.key }}</span>
                    </div>
                    <div style="display: flex; justify-content: space-between; height: 28px; gap: 16px; align-items: center;">
                        <span><b>Priority</b></span>
                        <span>{{ globalLink.priority }}</span>
                    </div>
                </div>
            </template>
            <el-switch
                class="unlinked"
                :modelValue="unlinked"
                @update:modelValue="emit('update:unlinked', $event as boolean)"
                :inactive-icon="Connection"
            />
        </el-tooltip>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import SelectElementType from "@c/select/SelectElementType"
import SelectSkillType from "@c/select/SelectSkillType"
import SelectMoonsignType from "@c/select/SelectMoonsignType"
import { useI18n } from "@/i18n/i18n"
import { Connection, QuestionFilled } from "@element-plus/icons-vue"
import type { GlobalLinkMeta } from "@/composables/config"

type ConfigItemParams = Record<string, any>

const props = withDefaults(defineProps<{
    modelValue?: any,
    type?: string,
    params?: ConfigItemParams,
    title?: string,
    globalLink?: GlobalLinkMeta,
    unlinked?: boolean,
}>(), {
    type: "",
    params: () => ({}),
    title: "",
    unlinked: false,
})

const emit = defineEmits<{
    (e: "update:modelValue", value: any): void,
    (e: "update:unlinked", value: boolean): void,
}>()

const { locale } = useI18n()

const currentLocale = computed(() => locale.value)

const currentOptions = computed(() => {
    if (locale.value.startsWith("zh")) {
        return props.params.options_zh
    }
    return props.params.options_en
})

function handleInputValue(value: string) {
    const parsedValue = value === "" ? 0 : parseFloat(value)
    handleChangeValue(Number.isNaN(parsedValue) ? 0 : parsedValue)
}

function handleChangeValue(value: any) {
    if (value !== props.modelValue) {
        emit("update:modelValue", value)
    }
}
</script>

<style lang="scss" scoped>
.config-item {
    position: relative;
}

.config-title {
    font-size: 12px;
    color: #666666;
    margin: 0 0 12px 0;
}

.unlinked {
    position: absolute;
    top: -15%;
    right: -5px;
}

.unlinked :deep(.el-switch__core) {
    display: none;
}
</style>
