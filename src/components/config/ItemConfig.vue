<template>
    <div class="config-root" :style="styleRoot">
<!--        {{ configs }}-->
        <ConfigItem
            class="config"
            v-for="config in configs.filter(c => c.type === 'globalLink')"
            :key="config.name"
            :params="config"
            :title="ta(config.title)"
            :type="config.type"
            :modelValue="value2[config.name]"
            @update:modelValue="handleInput(config.name, $event)"
            :name="config.name"
            :globalValue="globalValue"
            :globalConfigs="globalConfigs"
            :updateGlobalConfig="updateGlobalConfig"
            :unlinked="unlinked2[config.name]"
            @update:unlinked="handleUnlinked(config.name, $event)"
        ></ConfigItem>
        <ConfigItem
            class="config"
            v-for="config in configs.filter(c => c.type !== 'globalLink')"
            :key="config.name"
            :params="config"
            :title="ta(config.title)"
            :type="config.type"
            :modelValue="value2[config.name]"
            @update:modelValue="handleInput(config.name, $event)"
        ></ConfigItem>
    </div>
</template>

<script>
import { update } from "lodash";
import ConfigItem from "./ConfigItem"
import {useI18n} from "@/i18n/i18n"

export default {
    name: "ItemConfig",
    components: {
        ConfigItem
    },
    props: {
        modelValue: {},
        itemName: {},
        configs: {
            type: Array
        },
        globalValue: {
            required: false
        },
        globalConfigs: {
            type: Object,
            required: false
        },
        updateGlobalConfig: {
            type: Function,
            required: false
        },
        unlinked: {
            type: Object,
            required: false
        },
        bg: {
            default: "rgb(239, 246, 253)"
        },
        needItemName: {
            default: true,
        }
    },
    emits: ["update:modelValue", "update:unlinked"],
    computed: {
        styleRoot() {
            return {
                backgroundColor: this.bg
            }
        },

        value2() {
            if (this.needItemName) {
                return this.modelValue[this.itemName]
            } else {
                return this.modelValue
            }
        },

        unlinked2() {
            if (this.needItemName) {
                return this.unlinked[this.itemName]
            } else {
                return this.unlinked
            }
        }
    },
    
    methods: {
        handleInput(name, value) {
            if (this.needItemName) {
                let obj = Object.assign({}, this.modelValue[this.itemName])
                obj[name] = value

                this.$emit("update:modelValue", {
                    [this.itemName]: obj
                })
            } else {
                let obj = Object.assign({}, this.modelValue)
                obj[name] = value
                this.$emit("update:modelValue", obj)
            }
        },

        handleUnlinked(name, value) {
            if (this.needItemName) {
                let obj = Object.assign({}, this.unlinked[this.itemName])
                obj[name] = value

                this.$emit("update:unlinked", {
                    [this.itemName]: obj
                })
            } else {
                let obj = Object.assign({}, this.unlinked)
                obj[name] = value
                this.$emit("update:unlinked", obj)
            }
        }
    },
    setup() {
        const { t, ta } = useI18n()
        return {
            t, ta
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
