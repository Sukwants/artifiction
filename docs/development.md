<!-- 文档整理：Codex -->
# 开发与部署

## 环境与首次启动

Node.js 使用 22.12+；CI 使用 Node 22，浏览器测试依赖的 `puppeteer-core` 也有最低 Node 版本要求。Rust 使用根目录 `rust-toolchain.toml` 中的 stable。还需要 Git、wasm-pack 和 Rust 原生编译工具链；Windows 可使用 MSVC 对应的 C++ Build Tools。

从 Git 仓库克隆，而不是只下载源码 ZIP：`vue.config.js` 会读取 Git 提交号用于构建版本信息。

```sh
git clone --recursive https://github.com/Sukwants/artifiction.git genshin_artifact
cd genshin_artifact
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
npm ci
npm run build:wasm
npm run gen_meta
npm run serve
```

最后三步也可以用 `npm run dev` 一次完成。打开开发服务器实际输出的地址；端口被占用时可能与默认值不同。

已有检出缺少子模块时，运行 `git submodule update --init --recursive`。`sub/yas` 是独立扫描器，不参与本项目常规 WASM 构建；只有开发扫描器或更新它的提交引用时才需要处理其 Rust 环境。

## 改动后需要重建什么

| 改动 | 操作 |
| --- | --- |
| Vue 页面、组件、前端工具函数 | 开发服务器通常会热更新 |
| `mona_core/`、`mona_dsl/`、`mona_wasm/` 的计算逻辑 | `npm run build:wasm` |
| 角色、武器、圣遗物的名称、配置、说明、伤害列表等元数据 | `npm run gen_meta`，涉及计算时也重建 WASM |
| `mona_generate/` 的模板和生成逻辑 | `npm run gen_meta` |
| 构建 YAML、`vue.config.js`、依赖配置 | 重启开发服务器；依赖变更后同步锁文件 |

`npm run serve` 和 `npm run build` 不会自动执行 WASM 构建或元数据生成。新检出直接运行它们，可能遇到模块缺失。开发服务器也不会自动重新编译 Rust。

生成器写入 `src/assets/_gen_*.js` 和 `src/i18n/generated/{zh-cn,en}.json`。WASM 产物在 `mona_wasm/pkg/`。这些文件已被忽略：应修改 Rust 来源或生成模板，再重新生成。

前端依赖使用 npm 和 `package-lock.json`。Rust crate 各自有 manifest，运行 Cargo 时应指定对应目录或 `--manifest-path`；根目录没有 `Cargo.toml`。

## 代码与数据

修改游戏实现先阅读 [mona_docs 索引](../mona_docs/src/index.md)，再阅读对应模块规则。`prompt/` 用于维护者提供的原始游戏资料，目前被 Git 忽略，新检出中可能没有；缺少倍率、效果条件等依据时应明确列出缺项。

不要把圣遗物、收藏夹或计算预设的字段、ID、枚举顺序变化当作普通内部重构。它们可能影响既有浏览器数据、用户导出文件及页面 API。先检查读写双方，再确定兼容或迁移方案。

新增角色等内容通常需要实现代码、模块导出、名称枚举、配置解析及元数据一起更新，细节见对应核心文档。只修改生成后的 JavaScript 会在下次生成时丢失。

## 检查与 CI

[project-tests.yml](../.github/workflows/project-tests.yml) 当前由 pull request 触发，不是每次分支 push 都触发。流程依次安装依赖、构建 WASM、生成元数据、检查测试代码类型、构建前端、运行单元测试，最后启动开发服务器运行浏览器场景。

按改动范围选择本地验证，不必每次重复完整流程。若用户明确要求交给 CI，说明本地未运行以及 CI 的实际状态。

```sh
npm run build
npm run test:project:typecheck
npm run test:project:unit
```

浏览器场景需要先在另一个终端运行 `npm run serve`，然后执行：

```sh
npm run test:project:list
npm run test:project -- --suite project-smoke
```

`puppeteer-core` 不会随 `npm ci` 自动安装完整 Chrome；本地需要可用的 Chrome/Edge，必要时传入 `--browser-path`。`--debug` 会启动可见浏览器并在场景间暂停。[测试文档](../tests/README.md)说明了环境变量、错误分类和报告位置。CI 报告在 `project-test-results` artifact，本地失败现场在 `test-results/project/`。

`test:project:typecheck` 检查的是测试入口及其导入的 TypeScript，不能单凭这一项断言所有 Vue 页面都已验证。

## 构建与部署

```sh
npm run build:wasm
npm run gen_meta
npm run build
```

构建输出为 `dist/`，交给静态站点服务器托管。不要直接双击 `dist/index.html`：WASM、Worker 和路由需要正常的 HTTP 服务。

开发脚本选择 `.env.development.yaml`，生产脚本选择 `.env.production.yaml`，由 `ENV_FILE` 交给 `vue.config.js` 读取。常用配置：

| 配置 | 当前用途 |
| --- | --- |
| `MONA_TITLE` | 页面标题 |
| `MONA_ROUTE_MODE` | `hash` 或 `history` 路由 |
| `USE_CDN` | 是否从配置中的 CDN 加载部分前端库 |
| `MONA_NEED_BEIAN` | 是否展示备案相关内容 |
| `TAURI` | 桌面构建相关开关 |

当前开发配置为 hash 路由，生产配置为 history 路由。history 部署需要把前端路由（如 `/calculate`）回退到 `index.html`；静态资源和 `/api/` 应分别处理。WASM 文件应以 `application/wasm` 提供，子路径部署还需检查 `vue.config.js` 中的 `PublicPath`。

生产配置当前启用 CDN，因此部署产物仍可能需要联网加载这些资源。需要自包含资源时，在所选 YAML 中设置 `USE_CDN: false` 后重新构建。

分享链接等远程功能使用 `/api/`。开发代理配置在 `vue.config.js`，生产需要自行提供相应服务或代理。本仓库没有这套远程 API 的服务端。YAS 则由用户电脑上的本机服务处理，[接入说明](yas-web.md)列出了它与网页的关系。

根目录 `Dockerfile` 仍使用 Rust 1.59、Node 14 和旧目录结构，属于尚未同步的历史配置；当前构建基准是上面的 npm 流程与 CI，不能据此认为旧 Docker 镜像仍可直接构建。Tauri 有独立入口和 `.env.tauri.yaml`，常规前端 CI 不验证桌面打包。

## 常见问题

| 现象 | 首先检查 |
| --- | --- |
| 找不到 `mona_wasm/pkg` | 是否执行 `npm run build:wasm` |
| 找不到 `_gen_*` 或语言 JSON | 是否执行 `npm run gen_meta` |
| 修改 Rust 后计算结果没变 | 是否重新构建 WASM，浏览器是否仍缓存旧资源 |
| 角色或配置在页面上缺失 | 模块/枚举/元数据是否注册，是否重新生成元数据 |
| 提示找不到 Git 仓库 | 是否在真正的 Git 检出中构建 |
| 生产环境刷新页面 404 | history 路由是否配置回退 |
| 普通计算可用、分享失败 | `/api/` 服务或代理是否可用 |
| 浏览器测试无法启动 | Node 版本、浏览器路径及平台权限；查看测试报告 |
