<!-- 文档整理：Codex -->
# 文档目录

## 用户

- [使用说明](../src/pages/helps/InstructionPage/instruction.md)：账号、导入、计算、预设、备份与常见问题。该文件也是网页内「使用说明」的内容源。
- [YAS 网页扫描](yas-web.md)：连接包从哪里来、安装脚本做什么、如何连接和排查。
- [旧版图文手册](../mona_book/src/SUMMARY.md)：保留页面布局和操作示例，截图可能与当前版本不同。

## 维护者

- [开发与部署](development.md)：环境、生成步骤、部署配置、CI 和问题定位。
- [项目结构与数据流](architecture.md)：界面、Store、WASM、计算核心和扫描器的职责。
- [计算核心开发](../mona_docs/src/index.md)：角色、武器、圣遗物、Buff 与目标函数的实现约定。
- [页面自动化 API](../mona_api/mona-api.md)：`window.monaApi` 的实际接口。
- [测试框架](../tests/README.md)：场景注册、fixture 和运行方式。
- [MONA-DSL](../mona_dsl_book/src/SUMMARY.md)：自定义目标函数语言的说明。

## AI 协作

入口是根目录 [AGENTS.md](../AGENTS.md)。Copilot 使用同一份说明；计算核心的细节从 [mona_docs 索引](../mona_docs/src/index.md)按任务继续阅读。

## 文档维护约定

说明应指向实际实现或配置。更新命令、接口、数据格式、导入选项和 YAS 协议时，同时更新对应文档。

用户操作说明集中在网页的 `instruction.md`，不要另起一份完整副本。开发环境与构建步骤以 `development.md` 为详细说明，README 只保留入门命令。游戏实现规则继续放在 `mona_docs/`，避免通用文档复制所有角色规范。

遇到文档与代码不一致时，先核实当前代码、`package.json`、工具链与 CI 配置，再修正文档。旧教程可以提供背景，但不能用旧截图、旧示例或旧版本依赖推断当前行为。
