<template>
    <el-dialog title="使用 YAS 扫描圣遗物" :model-value="visible" width="80%"
        :close-on-click-modal="!scanning" :close-on-press-escape="!scanning" :show-close="!scanning"
        @update:model-value="emit('update:visible', $event)">
        <client-comp v-if="visible && !connected" :client="client" @done="onConnected" />
        <div v-if="connected">
            <p>请打开原神的圣遗物背包，拉到最上方。开始扫描后请等待完成，或用鼠标右键中止。</p>
            <el-alert v-if="error" :title="error" type="error" :closable="false" show-icon />
            <el-alert v-if="success" :title="success" type="success" :closable="false" show-icon />
            <el-form label-width="100px" :disabled="scanning">
                <el-form-item label="游戏窗口">
                    <el-select v-model="form.hwnd" placeholder="请选择游戏窗口">
                        <el-option v-for="(game, index) in windows" :key="game.hwnd" :label="`${game.title} · 窗口 ${index + 1}`" :value="game.hwnd" />
                    </el-select>
                    <el-button :loading="refreshing" @click="refreshWindows">刷新</el-button>
                </el-form-item>
                <el-form-item label="最小等级"><el-input-number v-model="form.minLevel" :min="0" :max="20" :precision="0" /></el-form-item>
                <el-form-item label="最小星级"><el-input-number v-model="form.minStar" :min="1" :max="5" :precision="0" /></el-form-item>
                <el-form-item label="扫描数量">
                    <el-input-number v-model="form.number" :min="0" :max="10000" :precision="0" />
                    <span class="hint">0 为不限数量</span>
                </el-form-item>
                <el-form-item>
                    <el-checkbox v-model="removeUnseen">删除未扫描到的圣遗物</el-checkbox>
                    <el-checkbox v-model="backupDir">备份“游戏中导入”收藏夹</el-checkbox>
                </el-form-item>
            </el-form>
            <el-alert v-if="removeUnseen" title="删除选项会移除网页中未出现在本次结果里的圣遗物，包括被扫描条件过滤的物品。"
                type="warning" :closable="false" show-icon />
            <div class="actions">
                <el-button type="primary" :disabled="scanning || refreshing || !form.hwnd" @click="startScan">开始扫描并导入</el-button>
                <el-button v-if="scanning" :loading="cancelling" :disabled="!job" @click="cancelScan">取消扫描</el-button>
                <el-button v-else @click="connected = false">重新连接</el-button>
            </div>
            <p v-if="scanning">{{ cancelling ? '正在取消扫描……' : '正在扫描，完成后会自动导入。' }}</p>
            <textarea v-if="logs.length" ref="outputArea" class="output" :value="logs.join('\n')" readonly aria-label="YAS 扫描日志" />
        </div>
    </el-dialog>
</template>

<script setup lang="ts">
import { markRaw, nextTick, onBeforeUnmount, reactive, ref, watch } from 'vue';
import ClientComp from './Client.vue';
import { delay, YasClient, type GameWindow } from './webcontrol';
import { validateMonaResult } from './result';
import { importMonaJson } from '@/utils/artifacts';
import { artifactsData } from '@/assets/artifacts';
import { artifactTags } from '@/constants/artifact';

const props = defineProps<{ visible: boolean }>();
const emit = defineEmits<{ (event: 'update:visible', value: boolean): void }>();
const client = markRaw(new YasClient());
const connected = ref(false);
const windows = ref<GameWindow[]>([]);
const form = reactive({ hwnd: 0, minLevel: 0, minStar: 5, number: 0 });
const removeUnseen = ref(false);
const backupDir = ref(false);
const refreshing = ref(false);
const scanning = ref(false);
const cancelling = ref(false);
const error = ref('');
const success = ref('');
const logs = ref<string[]>([]);
const outputArea = ref<HTMLTextAreaElement>();
const job = ref('');
let operation: AbortController | undefined;
let discardResult = false;

function message(cause: unknown) { return cause instanceof Error ? cause.message : String(cause); }
async function refreshWindows() {
    refreshing.value = true;
    error.value = '';
    const controller = new AbortController();
    operation?.abort();
    operation = controller;
    try {
        const games = await client.windows(controller.signal);
        if (controller.signal.aborted) return;
        windows.value = games;
        if (!windows.value.some(game => game.hwnd === form.hwnd)) form.hwnd = windows.value[0]?.hwnd ?? 0;
        if (!windows.value.length) error.value = '未找到原神窗口，请打开游戏后刷新';
    } catch (cause) { if (!controller.signal.aborted) error.value = message(cause); }
    finally { if (operation === controller) refreshing.value = false; }
}
function onConnected() { connected.value = true; void refreshWindows(); }

async function startScan() {
    operation?.abort();
    const controller = new AbortController();
    operation = controller;
    const options = { ...form };
    const deleteUnseen = removeUnseen.value;
    const backup = backupDir.value;
    scanning.value = true;
    discardResult = false;
    cancelling.value = false;
    error.value = '';
    success.value = '';
    logs.value = [];
    let after = 0;
    let failures = 0;
    let scanJob = '';
    try {
        scanJob = await client.scan(options, controller.signal);
        if (controller.signal.aborted) { await client.cancel(scanJob); return; }
        job.value = scanJob;
        while (!controller.signal.aborted) {
            let status;
            try { status = await client.status(scanJob, after, controller.signal); failures = 0; }
            catch (cause) {
                if (controller.signal.aborted || ++failures >= 3) throw cause;
                await delay(1000, controller.signal);
                continue;
            }
            after = status.next;
            logs.value = [...logs.value, ...status.logs.map(line => line.text)].slice(-2000);
            await nextTick();
            if (outputArea.value) outputArea.value.scrollTop = outputArea.value.scrollHeight;
            if (status.state === 'completed') {
                if (discardResult) { logs.value.push('已取消导入。'); break; }
                const result = await client.result(scanJob, controller.signal);
                if (controller.signal.aborted || discardResult) break;
                validateMonaResult(result, new Set(Object.keys(artifactsData)), new Set(Object.keys(artifactTags)));
                const imported = importMonaJson(result, deleteUnseen, backup, false);
                success.value = `扫描完成：新增 ${imported.add} 件，更新 ${imported.upgrade} 件，跳过 ${imported.skip} 件，删除 ${imported.remove} 件。`;
                break;
            }
            if (status.state === 'failed') throw new Error(status.error ?? '扫描失败，请查看日志');
            if (status.state === 'cancelled') { logs.value.push('扫描已取消，未导入数据。'); break; }
            await delay(500, controller.signal);
        }
    } catch (cause) {
        if (!controller.signal.aborted) error.value = message(cause);
        if (scanJob) {
            try { await client.cancel(scanJob); }
            catch { if (!controller.signal.aborted) error.value += '。无法连接 YAS，请在游戏中点击鼠标右键中止扫描。'; }
        }
    } finally {
        if (operation === controller) {
            job.value = '';
            scanning.value = false;
            cancelling.value = false;
        }
    }
}
async function cancelScan() {
    discardResult = true;
    cancelling.value = true;
    try { await client.cancel(job.value); }
    catch (cause) { error.value = message(cause); cancelling.value = false; }
}
function stop() {
    operation?.abort();
    if (job.value) void client.cancel(job.value).catch(() => {});
    operation = undefined;
    job.value = '';
    scanning.value = false;
    cancelling.value = false;
    refreshing.value = false;
}
watch(() => props.visible, visible => { if (!visible) { stop(); connected.value = false; } });
onBeforeUnmount(stop);
</script>

<style scoped>
.actions { display: flex; gap: 12px; margin: 20px 0; }
.actions .el-button { margin-left: 0; }
.hint { margin-left: 12px; color: var(--el-text-color-secondary); }
.output { width: 100%; height: 300px; box-sizing: border-box; resize: vertical; padding: 12px;
    border: 1px solid var(--el-border-color); border-radius: 4px; font-family: Consolas, monospace; }
</style>
