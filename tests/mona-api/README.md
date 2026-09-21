# Project browser test framework

这里保留底层 Puppeteer 会话框架和兼容示例。新的项目测试集应放在
`tests/suites/<suite-id>/suite.ts`，通过 `tests/run.ts` 统一发现和执行，详见
上级目录的 [测试集说明](../README.md)。

该目录提供基于 `puppeteer-core` 的浏览器测试框架。测试文件通过页面中的
`window.monaApi` 调用真实应用 API，不直接导入 Vue、Pinia 或 API 内部实现。

## 底层会话使用方式

先启动应用，例如：

```powershell
npm run serve
```

如果需要直接编写底层 Node 测试，可以使用：

```ts
import assert from "node:assert/strict"
import {test} from "node:test"
import {withMonaApiTest} from "./framework"

test("读取当前账号", async () => {
    await withMonaApiTest(async ({api}) => {
        const account = await api.accounts.getCurrent()
        assert.equal(typeof account.id, "number")
    }, {name: "accounts"})
})
```

当前锁定的 `puppeteer-core` 版本要求 Node 22.12+；运行时还需要 Chrome/Edge，以及
`puppeteer-core` 能访问的浏览器可执行文件。
当前项目的生产构建 Dockerfile 仍使用 Node 14，因此 E2E 测试应在独立的 Node 22+
环境运行，或后续单独升级 Dockerfile 的测试阶段。
默认连接 `http://127.0.0.1:8080`，也可以设置：

```powershell
$env:MONA_E2E_BASE_URL = "http://127.0.0.1:8080"
$env:MONA_E2E_BROWSER_PATH = "C:\Program Files\Google\Chrome\Application\chrome.exe"
npm run test:project
```

默认以无头模式启动浏览器。需要观察实际页面并在测试集之间手动检查状态时，
可以通过统一入口使用调试模式：

```powershell
npm run test:project -- --debug
```

调试模式会使用可见浏览器，并在每个测试集完成后等待终端任意按键；等待期间
浏览器保持打开。底层测试可以直接传入 `{headless: false}` 使用可见浏览器；
测试集之间的自动暂停由统一入口的 `--debug` 控制。`--browser-path` 或
`MonaApiTestOptions.executablePath` 可用于指定 Chrome/Edge 可执行文件。

框架会自动完成：

- 启动独立 BrowserContext，隔离 IndexedDB 数据；
- 打开应用并等待 `window.monaApi` 与 `monaApi.ready()`；
- 通过 `api.accounts.list()` 或 `api.call("accounts.list")` 调用接口；
- 通过 `navigate("calculator")` 调用页面导航 API；
- 将 API 错误转换为带 `code` 和 `details` 的 `MonaApiRemoteError`；
- 测试失败时保存截图、HTML 和诊断信息到 `test-results/project`。

`captureEvents("accounts.create", {name: "event-test"})` 可用于验证 API 事件。
