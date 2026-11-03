import assert from 'node:assert/strict';
import { test } from 'node:test';
import { YasClient, YasError, delay } from '../../src/pages/ArtifactsPage/YasUIDialog/webcontrol';
import { validateMonaResult } from '../../src/pages/ArtifactsPage/YasUIDialog/result';

test('YAS connection sends a fresh bearer token and typed scan options to loopback only', async context => {
    const requests: { url: string; init?: RequestInit }[] = [];
    context.mock.method(globalThis, 'fetch', async (input: string | URL | Request, init?: RequestInit) => {
        requests.push({ url: String(input), init });
        return new Response(JSON.stringify(String(input).endsWith('/api/info')
            ? { product: 'yas-web', protocolVersion: 1 }
            : { job: 'owned-job' }), { status: 200 });
    });
    const client = new YasClient();
    await client.connectRunning(new AbortController().signal);
    const connection = JSON.parse(requests[1].init!.body as string);
    assert.match(connection.token, /^[a-f0-9]{64}$/);
    const options = { hwnd: 123, minStar: 5, minLevel: 0, number: 0 };
    assert.equal(await client.scan(options), 'owned-job');
    const scan = requests[2];
    assert.equal(scan.url, 'http://127.0.0.1:32334/api/scan');
    assert.deepEqual(JSON.parse(scan.init!.body as string), options);
    assert.equal((scan.init!.headers as Record<string, string>).Authorization, `Bearer ${connection.token}`);
    assert.equal(scan.init!.credentials, 'omit');
    await client.connectRunning(new AbortController().signal);
    assert.notEqual(JSON.parse(requests[4].init!.body as string).token, connection.token);
    assert.ok(requests.every(request => request.url.startsWith('http://127.0.0.1:32334/')));
});

test('YAS refuses an incompatible service and preserves authorization failures', async context => {
    const fetch = context.mock.method(globalThis, 'fetch', async () =>
        new Response(JSON.stringify({ product: 'another-program', protocolVersion: 1 })));
    await assert.rejects(new YasClient().check(), /新版 YAS/);
    fetch.mock.mockImplementation(async () => new Response(JSON.stringify({ error: '拒绝授权' }), { status: 403 }));
    await assert.rejects(new YasClient().windows(), (error: unknown) => error instanceof YasError && error.status === 403);
});

test('closing a connection aborts its retry delay immediately', async () => {
    const controller = new AbortController();
    const waiting = delay(60000, controller.signal);
    controller.abort();
    await assert.rejects(waiting, { name: 'AbortError' });
});

const sets = new Set(['testSet']);
const stats = new Set(['lifeStatic', 'critical']);
function result() {
    return { flower: [{ setName: 'testSet', position: 'flower', star: 5, level: 20,
        mainTag: { name: 'lifeStatic', value: 4780 }, normalTags: [{ name: 'critical', value: 0.031 }] }],
    feather: [], sand: [], cup: [], head: [] };
}

test('YAS results require all five groups, supported items and a nonempty batch', () => {
    validateMonaResult(result(), sets, stats);
    assert.throws(() => validateMonaResult({}, sets, stats));
    assert.throws(() => validateMonaResult({ flower: [], feather: [], sand: [], cup: [], head: [] }, sets, stats), /没有扫描到/);
    const unsupported = result();
    unsupported.flower[0].setName = 'futureSet';
    assert.throws(() => validateMonaResult(unsupported, sets, stats));
    const malformed = result();
    malformed.flower.push({ ...malformed.flower[0], mainTag: { name: 'lifeStatic', value: Number.NaN } });
    const before = structuredClone(malformed);
    assert.throws(() => validateMonaResult(malformed, sets, stats));
    assert.deepEqual(malformed, before);
});
