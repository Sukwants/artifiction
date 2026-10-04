<!-- 文档整理：Codex -->
# Mona WASM

为前端提供 Rust 计算接口与优化应用，依赖同仓库中的 `mona_core` 和 `mona_dsl`。

从仓库根目录构建：

```sh
npm run build:wasm
```

也可以在本目录执行 `wasm-pack build`。工具链和首次开发准备见[开发文档](../docs/development.md)。

生成的 `pkg/` 由前端 `src/wasm/mona.ts` 加载，属于被 Git 忽略的构建产物。修改接口时同时检查 `src/wasm/` 调用封装和 `src/workers/` 的消息格式。角色、武器等页面元数据通过另一个步骤 `npm run gen_meta` 生成。

应用接口主要位于 `src/applications/`，核心效果位于 `../mona_core/`。[结构与数据流](../docs/architecture.md)提供定位入口，[计算核心文档](../mona_docs/src/index.md)提供业务约定。
