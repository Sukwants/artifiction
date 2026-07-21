<template>
    <div>
        <el-input
            v-if="enableSearch"
            v-model="searchString"
            style="margin-bottom: 16px"
            :placeholder="t('misc.search')"
            clearable
        >
            <template #append>
                <i-ep-search />
            </template>
        </el-input>

        <div class="config-items mona-scroll">
            <div
                v-for="item in dataSearched"
                :key="item.name"
                class="item"
            >
                <div class="top" v-if="hasEffect2(item) || hasEffect4(item)">
                    <img :src="item.thumbnail" class="image">
                    <div>
                        <h3 class="artifact-title">{{ item.title }}</h3>
                        <div>
                            <p v-if="hasEffect2(item)" style="font-size: 12px;">
                                <span class="effect-title">{{ t("misc.art2") }}</span>
                                <span class="effect-body">{{ item.effect2 }}</span>
                            </p>
                            <p v-if="hasEffect4(item)" style="font-size: 12px;">
                                <span class="effect-title">{{ t("misc.art4") }}</span>
                                <span class="effect-body">{{ item.effect4 }}</span>
                            </p>
                        </div>
                    </div>
                </div>

                <item-config
                    v-if="hasConfig2(item)"
                    :model-value="modelValue[item.config2Key]"
                    :configs="item.config2"
                    style="margin-bottom: 8px"
                />
                <item-config
                    v-if="hasConfig4(item)"
                    :model-value="modelValue[item.config4Key]"
                    :configs="item.config4"
                />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue"
import Fuse from "fuse.js"
import { artifactsData } from "@artifact"
import { toSnakeCase } from "@util/common"
import { getArtifactThumbnail } from "@util/artifacts"
import ItemConfig from "@/components/config/ItemConfig.vue"
import { useI18n } from "@/i18n/i18n"
import type { ConfigAddress, ConfigMeta } from "@/composables/config"

type ArtifactConfigEntry = {
    name: string,
    title: string,
    eng: string,
    config2Key: string,
    config4Key: string,
    config2: ConfigMeta[],
    config4: ConfigMeta[],
    effect2: string,
    effect4: string,
    count?: number,
    thumbnail: string,
}

const props = withDefaults(defineProps<{
    modelValue: Record<string, Record<string, ConfigAddress>>,
    enableSearch?: boolean,
    artifactSetCount?: Record<string, number>,
}>(), {
    enableSearch: true,
    artifactSetCount: () => ({}),
})

const { t, ta } = useI18n()
const searchString = ref("")

const data = computed<ArtifactConfigEntry[]>(() => {
    const results: ArtifactConfigEntry[] = []
    for (const name in artifactsData) {
        const d = (artifactsData as any)[name]
        const config4 = d.config4 ?? []
        const config2 = d.config2 ?? []
        const name2 = d.name2
        if (config4.length > 0 || config2.length > 0) {
            const snake = "config_" + toSnakeCase(name2)
            results.push({
                name: name2,
                title: ta(d.nameLocale),
                eng: d.eng,
                config2Key: `${snake}*set2`,
                config4Key: `${snake}*set4`,
                config4,
                config2,
                effect4: ta(d.effect4),
                effect2: ta(d.effect2),
                count: props.artifactSetCount[name],
                thumbnail: getArtifactThumbnail(name),
            })
        }
    }
    return results
})

const dataSearched = computed(() => {
    if (searchString.value === "") {
        return data.value
    }

    const fuse = new Fuse(data.value, {
        keys: ["title", "effect4", "effect2"],
    })
    return fuse.search(searchString.value).map(x => x.item)
})

function hasConfig2(item: ArtifactConfigEntry) {
    if (item.count != undefined) {
        return item.count >= 2 && item.config2.length > 0
    }
    return item.config2.length > 0
}

function hasConfig4(item: ArtifactConfigEntry) {
    if (item.count != undefined) {
        return item.count >= 4 && item.config4.length > 0
    }
    return item.config4.length > 0
}

function hasEffect2(item: ArtifactConfigEntry) {
    if (item.count != undefined) {
        return item.count >= 2
    }
    return true
}

function hasEffect4(item: ArtifactConfigEntry) {
    if (item.count != undefined) {
        return item.count >= 4
    }
    return true
}
</script>

<style scoped lang="scss">
.config-items {
    max-height: 60vh;

    .item {
        margin-bottom: 24px;

        &:last-of-type {
            margin-bottom: 0;
        }

        .top {
            display: flex;
            color: #606266;
        }

        .image {
            width: 64px;
            height: 64px;
            margin-right: 12px;
        }

        .effect-title {
            color: #6eb7ff;
        }

        .effect-body {
            word-break: normal;
        }

        .artifact-title {
            font-size: 12px;
            margin: 8px 0 0;
        }
    }
}
</style>
