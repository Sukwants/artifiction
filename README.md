<!-- 文档整理：Codex -->
# 叶师傅开锁铺

原神伤害计算与圣遗物配装工具。前端使用 Vue 3，计算核心使用 Rust，通过 WebAssembly 在浏览器中运行。

支持圣遗物导入与管理、单角色配装、整队优化、伤害明细、属性分析、圣遗物潜力分析，以及计算预设和自定义目标函数。计算结果取决于你设置的角色、技能、敌人和效果条件。

[English](README_en.md) · [日本語](README_ja.md)

## 从哪里开始

| 你要做什么 | 阅读入口 |
| --- | --- |
| 导入圣遗物、配装、备份数据 | [用户使用说明](src/pages/helps/InstructionPage/instruction.md) |
| 从网页直接启动 YAS 扫描 | [YAS 网页扫描说明](docs/yas-web.md) |
| 本地开发、构建和部署 | [开发与部署](docs/development.md) |
| 找到需要修改的代码 | [项目结构与数据流](docs/architecture.md) |
| 添加角色、武器、圣遗物效果 | [计算核心开发文档](mona_docs/src/index.md) |
| 写浏览器自动化或操作页面 API | [Mona API](mona_api/mona-api.md)、[测试框架](tests/README.md) |
| 让 AI 协助修改项目 | [AGENTS.md](AGENTS.md) |

完整导航见 [文档目录](docs/README.md)。网页内的「使用说明」也展示上表中的用户说明。

## 本地启动

准备 Node.js 22.12+、Rust stable、Git 和 wasm-pack。在 Windows 上构建 Rust 代码还需要相应的 C++ 编译工具链。

```sh
git clone --recursive https://github.com/Sukwants/artifiction.git genshin_artifact
cd genshin_artifact
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
npm ci
npm run dev
```

`npm run dev` 会构建 WASM、生成前端元数据，再启动开发服务器。之后只修改前端时可运行 `npm run serve`；修改 Rust 计算代码后，需要重新构建相应产物。详细步骤和常见问题见[开发文档](docs/development.md)。

项目使用 npm 与 `package-lock.json` 管理前端依赖。Rust crate 分别位于 `mona_core/`、`mona_wasm/` 等目录，根目录没有统一的 Cargo workspace。

## 数据和外部服务

圣遗物、收藏夹和预设按本地账号保存在当前网站的浏览器存储中。请定期导出备份；换域名、换浏览器或清理网站数据后，需要重新导入。

常规伤害计算和配装在浏览器中进行。分享链接等功能使用远程 `/api/` 服务，YAS 网页扫描使用单独的本机程序。仅部署前端并不会同时提供这些服务。

YAS 由[独立仓库](https://github.com/2745518585/yas)维护，本仓库的 `sub/yas` 固定到其中一个提交。网页连接需要含该功能的 YAS 发布包；若发布页还没有 `yas_web_*.zip`，可先使用扫描后的 `mona.json` 文件导入。

## 参与维护

提交问题时请附上复现步骤、预期与实际结果、所用页面和版本，以及尽可能小的预设或圣遗物样例。游戏数值问题还应说明技能、命座、效果触发条件和数据出处。提交前先移除样例中的个人备注。

开发与检查流程见[开发文档](docs/development.md)。目前项目 CI 在 pull request 上运行 WASM 构建、元数据生成、前端构建、类型检查、单元测试和浏览器场景；通过结果以实际 CI 记录为准。
