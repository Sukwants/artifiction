<template>
    <div class="config-root" :style="styleRoot">
        <ConfigItem
            v-for="configItem in renderConfigs"
            :key="configItem.meta.name"
            class="config"
            :params="configItem.meta"
            :title="configItem.title"
            :type="configItem.meta.type"
            :modelValue="configItem.value"
            :globalLink="configItem.meta.global_link"
            :unlinked="configItem.unlinked"
            @update:modelValue="configManager.updateConfigValue(configItem.address, $event)"
            @update:unlinked="configManager.updateUnlinkedStatus(configItem.address, $event)"
        />
    </div>
</template>

<script setup lang="ts">
import { computed, inject } from "vue"
import ConfigItem from "./ConfigItem.vue"
import { useI18n } from "@/i18n/i18n"
import { ConfigAddress, ConfigManager, type ConfigMeta } from "@/composables/config"

type RenderConfigItem = {
    meta: ConfigMeta,
    title: string,
    address: ConfigAddress,
    value: unknown,
    unlinked: boolean,
}

const props = withDefaults(defineProps<{
    modelValue: Record<string, ConfigAddress>,
    configs?: ConfigMeta[],
    bg?: string,
}>(), {
    configs: () => [],
    bg: "rgb(239, 246, 253)",
})

const configManager = inject<ConfigManager>("configManager")
if (!configManager) {
    throw new Error("ItemConfig requires a provided ConfigManager")
}

const { ta } = useI18n()

const styleRoot = computed(() => ({
    backgroundColor: props.bg,
}))

const renderConfigs = computed<RenderConfigItem[]>(() => {
    return props.configs
        .map(meta => {
            const address = props.modelValue?.[meta.name]
            if (!address) {
                return null
            }

            return {
                meta,
                title: ta(meta.title),
                address,
                value: configManager.getConfigValue(address),
                unlinked: configManager.getUnlinkedStatus(address),
            }
        })
        .filter((item): item is RenderConfigItem => item !== null)
})
</script>

<style lang="scss" scoped>
.config-root {
    padding: 12px;
    border-radius: 3px;

    .config {
        margin-bottom: 8px;

        &:last-of-type {
            margin-bottom: 0;
        }
    }
}
</style>
