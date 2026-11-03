<template>
    <div class="yas-client">
        <p>首次使用：下载网页连接包，解压后运行「安装网页连接.cmd」。</p>
        <p>
            <a :href="yasDownloadGithub" target="_blank" rel="noopener noreferrer">下载 YAS 网页连接包（yas_web_*.zip）</a>
        </p>
        <p>连接时请允许浏览器打开 YAS、访问本机网络，并在 YAS 的提示中允许当前网站。</p>
        <el-alert v-if="error" :title="error" type="error" :closable="false" show-icon />
        <div class="actions">
            <el-button type="primary" :disabled="connecting" @click="connect(true)">启动并连接 YAS</el-button>
            <el-button :disabled="connecting" @click="connect(false)">连接已运行的 YAS</el-button>
            <el-button v-if="connecting" @click="abort">停止等待</el-button>
        </div>
        <p v-if="connecting">正在等待 YAS 授权，请查看 Windows 的提示窗口……</p>
        <p v-else>支持 Windows 的原神客户端。也可以继续用 YAS 扫描后手动导入 mona.json。</p>
    </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, ref } from 'vue';
import links from '@/constants/links';
import { YasClient } from './webcontrol';

const props = defineProps<{ client: YasClient }>();
const yasDownloadGithub = links.yasDownloadGithub;
const emit = defineEmits<{ (event: 'done'): void }>();
const connecting = ref(false);
const error = ref('');
let pending: AbortController | undefined;

function abort() { pending?.abort(); }
async function connect(launch: boolean) {
    const controller = new AbortController();
    pending = controller;
    connecting.value = true;
    error.value = '';
    try {
        if (launch) {
            props.client.launch();
            await props.client.waitForConnection(controller.signal);
        } else {
            await props.client.connectRunning(controller.signal);
        }
        if (!controller.signal.aborted) emit('done');
    } catch (cause) {
        if (!controller.signal.aborted) {
            error.value = cause instanceof Error ? cause.message : '连接失败，请确认 YAS 已启动';
        }
    } finally {
        connecting.value = false;
    }
}
onBeforeUnmount(abort);
</script>

<style scoped>
.yas-client { line-height: 1.8; }
.actions { display: flex; flex-wrap: wrap; gap: 12px; margin-top: 20px; }
.actions .el-button { margin-left: 0; }
</style>
