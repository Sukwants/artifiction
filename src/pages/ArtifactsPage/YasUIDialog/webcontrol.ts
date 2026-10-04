const ENDPOINT = 'http://127.0.0.1:32334';

export interface GameWindow { hwnd: number; title: string }
export interface ScanOptions { hwnd: number; minStar: number; minLevel: number; number: number }
export interface ScanStatus {
    job: string;
    state: 'running' | 'cancelling' | 'cancelled' | 'completed' | 'failed';
    error: string | null;
    next: number;
    logs: { seq: number; text: string }[];
}

export class YasError extends Error {
    constructor(message: string, public readonly status: number) { super(message); }
}

export function delay(ms: number, signal: AbortSignal): Promise<void> {
    return new Promise((resolve, reject) => {
        const abort = () => { clearTimeout(timer); reject(new DOMException('操作已取消', 'AbortError')); };
        const timer = setTimeout(() => { signal.removeEventListener('abort', abort); resolve(); }, ms);
        if (signal.aborted) abort();
        else signal.addEventListener('abort', abort, { once: true });
    });
}

export class YasClient {
    private token = '';

    private newConnection(): void {
        this.token = Array.from(crypto.getRandomValues(new Uint8Array(32)), byte => byte.toString(16).padStart(2, '0')).join('');
    }

    // Call directly inside the click handler, before awaiting anything, to retain the user gesture.
    launch(): void {
        this.newConnection();
        const query = new URLSearchParams({ origin: window.location.origin, token: this.token });
        const link = document.createElement('a');
        link.href = `yas-scan://connect?${query.toString()}`;
        link.hidden = true;
        document.body.append(link);
        try { link.click(); } finally { link.remove(); }
    }

    private async request<T>(path: string, signal?: AbortSignal, body?: unknown, timeout = 8000): Promise<T> {
        const controller = new AbortController();
        const abort = () => controller.abort();
        const timer = setTimeout(abort, timeout);
        if (signal?.aborted) controller.abort();
        else signal?.addEventListener('abort', abort, { once: true });
        try {
            const response = await fetch(`${ENDPOINT}${path}`, {
                method: body === undefined ? 'GET' : 'POST',
                headers: { ...(this.token ? { Authorization: `Bearer ${this.token}` } : {}),
                    ...(body === undefined ? {} : { 'Content-Type': 'application/json' }) },
                body: body === undefined ? undefined : JSON.stringify(body),
                signal: controller.signal, credentials: 'omit', cache: 'no-store', referrerPolicy: 'no-referrer',
            });
            const value = await response.json();
            if (!response.ok) throw new YasError(value.error ?? 'YAS 请求失败', response.status);
            return value as T;
        } catch (error) {
            if (controller.signal.aborted && !signal?.aborted) {
                throw new Error('YAS 响应超时，请检查本机网络权限和 YAS 服务窗口');
            }
            if (error instanceof TypeError) throw new Error('无法连接 YAS，请确认服务已启动，并允许浏览器访问本机网络');
            throw error;
        } finally {
            clearTimeout(timer);
            signal?.removeEventListener('abort', abort);
        }
    }

    async check(signal?: AbortSignal): Promise<void> {
        const info = await this.request<{ product: string; protocolVersion: number }>('/api/info', signal);
        if (info.product !== 'yas-web' || info.protocolVersion !== 1) {
            throw new YasError('请安装支持网页扫描的新版 YAS', 426);
        }
    }

    async connectRunning(signal: AbortSignal): Promise<void> {
        this.newConnection();
        await this.check(signal);
        await this.request('/api/connect', signal, { token: this.token }, 120000);
    }

    async waitForConnection(signal: AbortSignal): Promise<void> {
        const deadline = Date.now() + 120000;
        while (!signal.aborted && Date.now() < deadline) {
            try {
                await this.check(signal);
                await this.request('/api/session', signal);
                return;
            } catch (error) {
                if (signal.aborted || (error instanceof YasError && [403, 426].includes(error.status))) throw error;
            }
            await delay(1000, signal);
        }
        throw new Error('连接超时。请确认已安装网页连接包，并允许浏览器打开 YAS、访问本机网络和 YAS 的授权提示。');
    }

    windows(signal?: AbortSignal): Promise<GameWindow[]> { return this.request('/api/windows', signal); }
    async scan(options: ScanOptions, signal?: AbortSignal): Promise<string> {
        const response = await this.request<{ job: string }>('/api/scan', signal, options);
        return response.job;
    }
    status(job: string, after: number, signal?: AbortSignal): Promise<ScanStatus> {
        return this.request(`/api/status?${new URLSearchParams({ job, after: String(after) })}`, signal);
    }
    result(job: string, signal?: AbortSignal): Promise<unknown> {
        return this.request(`/api/result?${new URLSearchParams({ job })}`, signal);
    }
    async cancel(job: string): Promise<void> { await this.request('/api/cancel', undefined, { job }); }
}
