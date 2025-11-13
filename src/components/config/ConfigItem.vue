<template>
    <div>
<!--        <h3 class="config-title">{{ // params.title }}</h3>-->
        <h3 class="config-title" v-if="type !== 'globalLink'">{{ title }}</h3>
        <template v-if="type === 'float'">
            <el-slider
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="0.1"
                :show-input="true"
                :show-input-controls="false"
            ></el-slider>
        </template>
        <template v-if="type === 'int'">
            <el-slider
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
                :step="1"
            ></el-slider>
        </template>
        <template v-if="type === 'intInput'">
            <el-input-number
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :min="params.min"
                :max="params.max"
            ></el-input-number>
        </template>
        <template v-if="type === 'bool'">
            <el-switch
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :active-text="currentLocale.startsWith('zh') ? '是' : 'On'"
            ></el-switch>
        </template>
        <template v-if="type === 'floatPercentageInput'">
            <el-input
                :modelValue="modelValue"
                @update:modelValue="handleInputValue"
            >
                <template slot="append">%</template>
            </el-input>
        </template>
        <template v-if="type === 'floatInput'">
            <el-input
                :modelValue="modelValue"
                @update:modelValue="handleInputValue"
            >
            </el-input>
        </template>
        <template v-if="type === 'element4'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="['Pyro', 'Cryo', 'Electro', 'Hydro']"
            ></select-element-type>
        </template>
        <template v-if="type === 'element8'">
            <select-element-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :elements="['Pyro', 'Cryo', 'Electro', 'Hydro', 'Anemo', 'Geo', 'Dendro', 'Physical']"
            ></select-element-type>
        </template>
        <template v-if="type === 'element8multi'">
            <select-element-multi
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
            ></select-element-multi>
        </template>
        <template v-if="type === 'skill4'">
            <select-skill-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
            ></select-skill-type>
        </template>
        <template v-if="type === 'option'">
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
        <template v-if="type === 'option2'">
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
        <template v-if="type === 'moonsign2'">
            <select-moonsign-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :moonsigns="['Nascent', 'Ascendant']"
            ></select-moonsign-type>
        </template>
        <template v-if="type === 'moonsign3'">
            <select-moonsign-type
                :modelValue="modelValue"
                @update:modelValue="handleChangeValue"
                :moonsigns="['None', 'Nascent', 'Ascendant']"
            ></select-moonsign-type>
        </template>
        <template v-if="type === 'globalLink'">
            <div class="global-config-item">
                <ConfigItem
                    class="config"
                    v-if="type === 'globalLink' && !unlinked"
                    :key="'[global]' + name"
                    :params="params.config"
                    :title="title"
                    :type="params.config.type"
                    :modelValue="globalValue"
                    @update:modelValue="updateGlobalConfig(params.key, $event)"
                ></ConfigItem>
                <ConfigItem
                    class="config"
                    v-if="type === 'globalLink' && unlinked"
                    :key="'[local]' + name"
                    :params="params.config"
                    :title="title"
                    :type="params.config.type"
                    :modelValue="modelValue"
                    @update:modelValue="handleChangeValue"
                ></ConfigItem>
                <el-switch
                    class="unlinked"
                    :modelValue="unlinked"
                    @update:modelValue="$emit('update:unlinked', $event)"
                    :inactive-text="'linked'"
                    :size="'small'"
                ></el-switch>
            </div>
        </template>
    </div>
</template>

<script>
import SelectElementType from "@c/select/SelectElementType"
import SelectSkillType from "@c/select/SelectSkillType"
import SelectMoonsignType from "@c/select/SelectMoonsignType"
import { useI18n } from "@/i18n/i18n"

export default {
    name: "ConfigItem",
    components: { SelectSkillType, SelectElementType, SelectMoonsignType },
    props: {
        modelValue: {},
        type: {},
        params: {},
        title: {},
        name: {
            type: String,
            required: false
        },
        globalValue: {},
        updateGlobalConfig: {
            type: Function,
            required: false
        },
        unlinked: {
            type: Boolean,
            required: false
        }
    },
    emits: ["update:modelValue", "update:unlinked"],
    computed: {
        currentLocale() {
            const { locale } = useI18n()
            return locale.value
        },
        currentOptions() {
            const { locale } = useI18n()
            if (locale.value.startsWith('zh')) {
                return this.params.options_zh;
            } else {
                return this.params.options_en;
            }
        }
    },
    methods: {
        handleInputValue(value) {
            let v = 0
            if (value === "") {
                v = 0
            } else {
                v = parseFloat(value)
                if (isNaN(v)) {
                    v = 0
                }
            }
            this.handleChangeValue(v)
        },
        handleChangeValue(value) {
            if (value !== this.modelValue) {
                this.$emit("update:modelValue", value)
            }
        },
    }
}
</script>

<style lang="scss" scoped>
.config-title {
    font-size: 12px;
    color: #666666;
    margin: 0 0 12px 0;
}

.global-config-item {
    position: relative;
}

.unlinked {
    position: absolute;
    top: -4%;
    right: 0;
}

.unlinked :deep(.el-switch__core) {
    display: none;
}

// .config-item {
//     margin-bottom: 8px;

//     &:last-of-type {
//         margin-bottom: 0;
//     }
// }
</style>
