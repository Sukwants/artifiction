<template>
    <el-radio-group
        :model-value="normalizedValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-radio-button v-if="statesMap.has('None')" label="None">{{ t("stellar_glimmer_state.None") }}</el-radio-button>
        <el-radio-button v-if="statesMap.has('StellarConduct')" label="StellarConduct">{{ t("stellar_glimmer_state.StellarConduct") }}</el-radio-button>
        <el-radio-button v-if="statesMap.has('StellarSwirl')" label="StellarSwirl">{{ t("stellar_glimmer_state.StellarSwirl") }}</el-radio-button>
    </el-radio-group>
</template>

<script>
import {useI18n} from "../../i18n/i18n"

export default {
    name: "SelectStellarGlimmerType",
    emits: ["update:modelValue"],
    props: {
        modelValue: {
            type: [String, Number],
            default: "None",
        },
        states: {
            default: () => {
                return ["None", "StellarConduct", "StellarSwirl"];
            }
        },
    },
    computed: {
        statesMap() {
            let temp = new Set();
            for (let i of this.states) {
                temp.add(i);
            }
            return temp;
        },
        // 兼容旧的数字配置（0=无，1=辉映·星超导，2=辉映·星扩散）
        normalizedValue() {
            if (typeof this.modelValue === "number") {
                return ["None", "StellarConduct", "StellarSwirl"][this.modelValue] || "None";
            }
            return this.modelValue;
        }
    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
}
</script>
