<!-- 文档整理：Codex -->
# AI 协作说明

本文件适用于整个仓库。按用户本次请求的范围工作；更深目录的说明补充对应模块规则。先检查工作区状态，保留已有的用户改动。

## 阅读顺序

1. 从 `mona_docs/src/index.md` 进入，按任务选择文档。
2. 前端、存储、构建或扫描接入，阅读 `docs/architecture.md` 和相关的 `docs/development.md`、`docs/yas-web.md`。
3. 角色、武器、圣遗物、Buff 或目标函数，阅读 `mona_docs` 中对应模块的索引和引用文档。涉及属性或伤害时阅读 `attribute.md`、`config.md`、`damage_builder.md`。
4. 操作真实页面先阅读 `mona_api/mona-api.md`，等待 `api.ready()` 并确认目标页面已挂载；添加浏览器场景先阅读 `tests/README.md`。

只读与任务相关的实现和示例，搜索优先使用 `rg`。不要因任务涉及一个页面就遍历所有游戏实现，也不要把历史 README、注释掉的入口或旧截图当成当前功能依据。

## 项目事实

- Vue 3 前端位于 `src/`；Rust 计算核心位于 `mona_core/`，WASM 应用位于 `mona_wasm/`。
- 前端依赖使用 npm 和 `package-lock.json`。不要引入其他包管理器的锁文件。
- 根目录没有 Cargo workspace；使用对应 crate 的 manifest。根目录工具链是 stable，YAS 有独立环境。
- `src/store/pinia/` 当前是 Vue 响应式对象与模块级单例，不要仅凭目录名假定 Pinia API。
- `mona_wasm/pkg/`、`src/assets/_gen_*.js`、`src/i18n/generated/*.json` 为生成产物；修改来源或模板后生成。
- `prompt/` 是被 Git 忽略的原始资料目录，新检出可能缺失。需要游戏数值时先查这里；未找到时明确报告缺少什么，不编造倍率、效果或技能条件。
- `sub/yas` 是独立仓库的固定提交引用。修改其源码、更新引用和发布程序需要分别处理。

## 修改边界

复用现有 Store、WASM 调用封装和 `window.monaApi`。修改异步流程时处理初始化、重复请求、取消、超时、卸载和资源释放。

导入数据、账号切换、收藏夹 ID、名称枚举和持久化字段影响用户已有数据。修改前核实所有读写入口，说明兼容或迁移方式。名称枚举遵守对应核心文档的追加约定。

游戏实现的条件、倍率与默认值需要数据依据。难以表达的效果按模块规则处理；使用近似时在对应代码中明确含义。业务不确定性应列出具体待确认项，已授权且可确定的工作继续完成。

重构控制在授权范围，避免无关的批量格式化和整仓库改写。依赖变化才更新对应锁文件。不要手工编辑生成文件来掩盖来源问题，也不要把本机绝对路径写进跨平台脚本。

用户仅要求审查时，按 `mona_docs/src/review/index.md` 输出发现；用户授权修复时可以完成相应修改。不要因文档中的通用提醒重新索要已给出的授权。

## 验证与交付

常用命令：

```sh
npm ci
npm run build:wasm
npm run gen_meta
npm run serve
npm run build
npm run test:project:typecheck
npm run test:project:unit
npm run test:project -- --suite project-smoke
```

`serve`、`build` 不会预先生成 WASM 或元数据；首次启动需先生成，或使用 `npm run dev`。浏览器场景需有开发服务器及可用浏览器，详见测试文档。

按用户要求和变更风险选择验证。文档改动检查链接、路径和命令依据即可；用户明确要求交给 CI 时，不额外运行本地测试。当前项目 CI 在 pull request 上触发，不要把「已配置检查」「本地某项通过」写成「CI 已通过」。

最终说明改了什么、验证了什么、还缺什么。外部服务、生成文件缺失、未运行验证或发布包尚未生成等限制如实说明。只在用户要求时提交或推送，提交内容应包含本次相关改动。
