<template>
    <el-radio-group
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-radio-button v-if="moonsignMap.has('None')" label="None">{{ t("moonsign.None") }}</el-radio-button>
        <el-radio-button v-if="moonsignMap.has('Nascent')" label="Nascent">{{ t("moonsign.Nascent") }}</el-radio-button>
        <el-radio-button v-if="moonsignMap.has('Ascendant')" label="Ascendant">{{ t("moonsign.Ascendant") }}</el-radio-button>
    </el-radio-group>
</template>

<script>
import {useI18n} from "../../i18n/i18n"

export default {
    name: "SelectMoonsignType",
    emits: ["update:modelValue"],
    props: {
        modelValue: {
            type: String,
            required: true,
        },
        moonsigns: {
            default: () => {
                return ["None", "Nascent", "Ascendant"];
            }
        },
        clearable: {
            default: false,
        }
    },
    computed: {
        moonsignMap() {
            let temp = new Set();
            for (let i of this.moonsigns) {
                temp.add(i);
            }
            return temp;
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