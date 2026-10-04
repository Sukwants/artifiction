<!-- 文档整理：Codex -->
# 项目结构与数据流

## 目录地图

| 目录或文件 | 职责 |
| --- | --- |
| `src/main.ts`、`src/App.vue` | 挂载 Vue 应用、语言和页面自动化 API |
| `src/router/router.js` | 页面入口与路由 |
| `src/pages/`、`src/components/` | 功能页面与共享组件 |
| `src/composables/` | 计算器等页面使用的响应式配置 |
| `src/store/pinia/` | 圣遗物、收藏夹、预设、账号的响应式 Store |
| `src/store/backend.ts` | 浏览器存储及文件目录后端实现 |
| `src/wasm/`、`src/workers/` | 前端计算调用封装与 Worker |
| `src/api/mona/` | `window.monaApi` 的页面操作接口 |
| `src/api/client.js`、`src/api/repo.js` | 远程 `/api/` 请求与分享内容 |
| `mona_core/` | 角色、武器、圣遗物效果、属性与伤害计算 |
| `mona_wasm/` | WASM 导出接口、配装优化与潜力等应用算法 |
| `mona_dsl/`、`mona_derive/` | 自定义目标函数语言与 Rust 派生宏 |
| `mona_generate/` | 把 Rust 元数据和文案生成前端资源 |
| `src/assets/`、`src/i18n/` | 图片、元数据入口与语言资源 |
| `tests/` | Node 单元测试、场景框架和浏览器场景 |
| `sub/yas` | 独立 YAS 仓库的固定提交引用 |

`src/store/pinia/` 是历史目录名，当前实现使用 Vue 的 `ref`、`reactive`、`computed` 与模块级单例。阅读 Store 时应以这些文件的实际接口为准；不要仅凭目录名引入新的状态库或改写状态模型。

## 从点击到计算结果

页面收集角色、武器、敌人、Buff 和圣遗物设置，通过 `src/wasm/` 调用计算接口。部分计算直接使用 WASM；耗时优化通过 Worker 运行，结果再由 Vue 展示。

通用 Worker 请求生命周期在 `src/wasm/worker_request.js`：等待 `ready`，发送一次请求，接收 `result` 或 `error`，超时或结束后释放监听器、计时器和 Worker。修改 Worker 协议时，应同步检查调用封装和 `src/workers/` 中的消息格式。

计算效果和注册信息主要来自 `mona_core/`；优化应用与导出接口位于 `mona_wasm/src/applications/`。例如伤害显示错误应同时检查页面输入、WASM 输入转换和核心效果，不能只改展示数字。

## 从 Rust 元数据到页面选项

`mona_generate/src/bin/gen_meta.rs` 从核心实现生成 `src/assets/_gen_*.js`，包括角色、武器、圣遗物、Buff、目标函数等信息，同时生成语言 JSON。前端资源入口再引用这些产物。

因此新增效果时，计算逻辑与页面配置可能需要同时更新，并分别重建 WASM 和元数据。生成文件、`mona_wasm/pkg/` 和各 crate 的 `target/` 不属于需要手工维护的源码。

## 数据保存与账号切换

`src/store/pinia/account.ts` 初始化当前账号的圣遗物、收藏夹和预设，并监听它们的变化进行持久化。浏览器后端通过 localforage 工作；语言等部分设置使用 localStorage。数据按网站来源和本地账号隔离。

账号切换会先等待后端写入，再加载新账号的数据。处理异步入口时可使用已有的 Store 就绪接口；避免在初始化期间写回空状态。

`backend.ts` 还保留 File System Access 的目录后端，但当前账号页的同步按钮和对话框已被注释。实现存在不等于用户界面已经开放，用户文档不能教用户点击这些隐藏入口。

## 导入、导出与分享

圣遗物导入由 `src/utils/artifacts.ts` 中的 `importMonaJson` 处理，识别五个部位分组，结合哈希判断已有或升级物品，并处理收藏夹与删除选项。它会修改 Store，调用方需要在进入前确定数据和选项。

网页导出包括五个分组与 `kumi`；计算预设独立导出。文件导入、页面 API 导入与 YAS 自动导入是不同入口，需要分别核实校验与默认选项，不能把某一个入口的保护推断为其他入口也具备。

分享链接通过远程 `/api/repo/create` 上传内容，读取分享则请求 `/api/repo/<code>`。浏览器本地保存、文件备份和远程分享是不同的数据路径。

## YAS 网页扫描

`src/pages/ArtifactsPage/YasUIDialog/` 负责连接、扫描选项、日志轮询、取消、结果校验和导入。网页通过 `yas-scan://` 唤起程序，再连接 `127.0.0.1:32334`，不接受网页传入任意命令行。

扫描服务实现属于 YAS 仓库；本仓库只维护网页客户端和子模块提交引用。源码修改、子模块引用更新、发布连接包和部署网页是分别需要完成的步骤，详见 [YAS 接入](yas-web.md)。

## 页面 API 与测试

`src/main.ts` 安装 `window.monaApi`。API 操作现有 Store 和页面响应式状态，自动化修改会真实反映到页面。接口文档在 [mona-api.md](../mona_api/mona-api.md)。

项目场景入口是 `tests/suites/*/suite.ts`；框架使用隔离的浏览器上下文，通过页面 API 操作应用。Node 单元测试放在 `tests/unit/`。添加浏览器场景时优先复用 `tests/framework/`，不要为同一页面重新创建另一套自动化接口。
