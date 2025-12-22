<template>
    <div class="doc-page">
        <div class="doc-content markdown-body">
            <Text />
        </div>
        <div class="doc-toc">
            <el-anchor :offset="80" :affix="true" :show-ink="true">
                <el-anchor-link v-for="h1 in headings" :key="h1.id" :href="'#' + h1.id" :title="h1.title">
                    <template #sub-link>
                        <el-anchor-link
                            v-for="h2 in h1.children"
                            :key="h2.id"
                            :href="'#' + h2.id"
                            :title="h2.title"
                        ></el-anchor-link>
                    </template>
                </el-anchor-link>
            </el-anchor>
        </div>
    </div>
</template>

<script>
import Text from './instruction.md'
import 'github-markdown-css/github-markdown.css'
import { ref, onMounted } from 'vue'

const headings = ref([])

export default {
    components: {
        Text,
    },
    setup() {
        onMounted(() => {
            const element = document.querySelectorAll('.markdown-body h1, .markdown-body h2')
            console.log(element)
            let list = []
            for (const h of element) {
                if (h.localName == 'h1') {
                    list.push({
                        id: h.id,
                        title: h.innerText,
                        children: [],
                    })
                } else {
                    if (list.length === 0) return
                    list[list.length - 1].children.push({
                        id: h.id,
                        title: h.innerText,
                    })
                }
            }
            console.log(list)
            headings.value = list
        })
    },
    computed: {
        headings() {
            return headings.value
        },
    },
}
</script>

<style scoped lang="scss">
.doc-page {
    display: flex;
    gap: 2rem;
}

.doc-content {
    flex: 1;
    padding-right: 20%;
}

.doc-toc {
    width: 15%;
    height: 80%;
    overflow-y: auto;
    position: fixed;
    top: 80px;
    right: 0px;
    border-left: 1px solid #eee;
    padding-left: 1rem;
}
</style>