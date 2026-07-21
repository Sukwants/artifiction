<template>
    <div class="config-root" :style="styleRoot">
<!--        {{ configs }}-->
        <div v-for="config in configs">
            <ConfigItem
                class="config"
                v-if="config.type === 'globalLink'"
                :key="config.name"
                :params="config"
                :title="ta(config.config.title)"
                :type="config.type"
                :modelValue="value2[config.name].config"
                @update:modelValue="handleInput(config.name, $event)"
                :name="config.name"
                :globalValue="getConfigItemValue(value2[config.name])"
                :updateGlobalConfig="updateGlobalConfig"
                :unlinked="value2[config.name].unlinked"
                @update:unlinked="handleUnlinked(config.name, $event)"
            ></ConfigItem>
            <ConfigItem
                class="config"
                v-if="config.type !== 'globalLink'"
                :key="config.name"
                :params="config"
                :title="ta(config.title)"
                :type="config.type"
                :modelValue="value2[config.name].config"
                @update:modelValue="handleInput(config.name, $event)"
            ></ConfigItem>
        </div>
    </div>
</template>

<script lang="ts">
import ConfigItem from "./ConfigItem"
import {useI18n} from "@/i18n/i18n"
import { getConfigItemValue } from "@/composables/globalConfig"
import { deepCopy } from "@/utils/common"
import { ConfigManager, ConfigAddress } from "@/composables/config"
import { PropType } from "vue"

const configManager = inject<ConfigManager>("configManager")!

export default {
    name: "ItemConfig",
    components: {
        ConfigItem
    },
    props: {
        modelValue: {
            type: Object as PropType<Record<string, ConfigAddress>>,
            required: true
        },
        configs: {
            type: Array
        },
        bg: {
            default: "rgb(239, 246, 253)"
        },
    },
    emits: ["update:modelValue"],
    computed: {
        styleRoot() {
            return {
                backgroundColor: this.bg
            }
        },

        value2() {
            return configManager.getObjectValue(this.modelValue)
        }
    },
    
    methods: {
        handleInput(name, value) {
            if (this.needItemName) {
                let obj = deepCopy(this.modelValue[this.itemName])
                obj[name] = {
                    ...obj[name],
                    config: value
                }

                this.$emit("update:modelValue", {
                    [this.itemName]: obj
                })
            } else {
                let obj = deepCopy(this.modelValue)
                obj[name] = {
                    ...obj[name],
                    config: value
                }
                this.$emit("update:modelValue", obj)
            }
        },

        handleUnlinked(name, value) {
            if (this.needItemName) {
                let obj = deepCopy(this.modelValue[this.itemName])
                obj[name] = {
                    ...obj[name],
                    unlinked: value
                }

                this.$emit("update:modelValue", {
                    [this.itemName]: obj
                })
            } else {
                let obj = deepCopy(this.modelValue)
                obj[name] = {
                    ...obj[name],
                    unlinked: value
                }
                this.$emit("update:modelValue", obj)
            }
        }
    },
    setup() {
        const { t, ta } = useI18n()
        return {
            t, ta, getConfigItemValue
        }
    }
}
</script>

<style lang="scss" scoped>
.config-root {
    padding: 12px;
    border-radius: 3px;
    // background-color: rgb(239, 246, 253);

    .config {
        margin-bottom: 8px;
        &:last-of-type {
            margin-bottom: 0;
        }
    }
}
</style>
