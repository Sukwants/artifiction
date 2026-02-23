<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
        multiple
        collapse-tags
        collapse-tags-tooltip
        max-collapse-tags="3"
        :placeholder="t('calcPage.noTag')"
        tag-type="primary"
    >
        <el-option
            v-for="tag in tags"
            :key="tag.name"
            :label="ta(tag.nameLocale)"
            :value="tag.name"
        >
        </el-option>
    </el-select>
</template>

<script>
import {useI18n} from "@/i18n/i18n";

export default {
    name: "SelectTag",
    props: {
        modelValue: {
            type: Array,
            required: true,
        },
        tags: {
            type: Array,
            required: true,
        },
    },
    emits: ["update:modelValue"],
    setup() {
        const { t, ta } = useI18n()

        return {
            t,
            ta
        }
    },
}

</script>