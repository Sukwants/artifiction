# 项目测试集

测试集统一放在 `tests/suites` 下。每个一级子目录代表一个测试集，并且必须提供
`suite.ts` 作为入口：

```text
tests/
├─ framework/       # 公共测试框架
├─ suites/
│  ├─ project-smoke/
│  │  └─ suite.ts
│  ├─ flins-thunderous-symphony/
│  │  ├─ suite.ts         # 只描述顺序操作
│  │  └─ fixtures/        # 原始导出数据
│  └─ hexerei-four-character-team/
│     ├─ suite.ts
│     └─ fixtures/        # 原始预设与圣遗物导出数据
└─ run.ts            # 所有测试集的统一入口
```

每个测试集只负责一个相对独立的项目任务。浏览器中的 `monaApi` 只是操作项目的
控制入口，不代表测试目标。菲林斯伤害场景单独放在
`flins-thunderous-symphony`，不会和项目基础冒烟测试混在一起。

## 注册测试集

```ts
import {defineProjectTestSuite, projectSteps, projectTestCase} from "../../framework"

export default defineProjectTestSuite({
    id: "calculator",
    name: "计算器测试集",
    cases: [
        projectTestCase(import.meta.url, {
            id: "basic-calculation",
            category: "calculation",
            operations: [
                projectSteps.enter(),
                projectSteps.applyPreset("某个预设"),
                projectSteps.selectSkill(0),
                projectSteps.expectDamage("Damage", "critical", 100, 0.05)
            ]
        })
    ]
})
```

`operations` 按顺序执行，每一行就是一个项目操作。导入、页面跳转、预设与套装应用、
伤害计算和断言都由框架实现；操作失败会抛出带类别和详情的错误，运行器再附加测试集、
用例等元数据并汇总结果。

fixture 直接保存应用可导入的原始 JSON：圣遗物文件是按五个部位分组的对象，预设文件是
预设数组；不再需要 `schema`、`data` 等测试专用包装字段。圣遗物导出文件中的 `kumi`
套装目录会由框架自动用于按名称应用套装。伤害期望直接写在
`expectDamage` 操作中，最后一个参数是相对误差比例，`0.05` 表示 5%。

队伍测试可以用 `addCharacter`、`applyPreset` 和带角色名参数的
`applyArtifactSet` 描述多角色配置；`setConfig` 用配置地址设置角色或圣遗物配置，
`setEnemyLevel` 可在计算前调整怪物等级。
计算器默认提供一个初始角色槽，测试集第一次调用 `addCharacter` 时会复用这个槽，
后续调用才会新增角色，因此不会额外留下未配置角色。

发现器只扫描 `tests/suites/*/suite.ts`，所以公共框架和普通测试文件不会被误认为测试集。
如果测试集目录缺少 `suite.ts`，统一结果会返回 `test-file-missing`。

## 运行

```powershell
# 只发现并列出测试集，不启动浏览器
npm run test:project -- --list

# 运行全部测试集
npm run test:project

# 只运行指定测试集
npm run test:project -- --suite flins-thunderous-symphony

# 调试模式：使用可见浏览器，并在每个测试集结束后暂停
npm run test:project -- --debug

# 调试模式下指定 Chrome/Edge 可执行文件
npm run test:project -- --debug --browser-path "C:\Program Files\Google\Chrome\Application\chrome.exe"

# 发现或执行失败后立即停止
npm run test:project -- --fail-fast
```

默认使用无头浏览器。`--debug` 会打开可见的 Chrome/Edge 窗口，并在每个测试集
完成后暂停，等待终端任意按键；按键前当前测试集的浏览器和页面状态都会保留。
`--headless` 可显式使用无头模式。`--browser-path`（也支持 `--browser`）用于
指定浏览器路径，优先级高于自动探测和 `MONA_E2E_BROWSER_PATH`。

结果同时包含 `type` 和 `severity`。例如：`passed`、`calculation-error`、
`damage-out-of-tolerance`、`page-crashed`、`fixture-missing`、`test-file-missing`、
`suite-import-error` 和 `framework-error`。测试用例返回的对象会保存在结果的
`details` 字段中，便于读取实际伤害、预期伤害、绝对差值和相对误差比例。
