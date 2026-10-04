<!-- Documentation updated by Codex. -->
# 叶师傅开锁铺 (Artifiction)

A Genshin Impact damage calculator and artifact optimizer. The Vue 3 frontend uses a Rust core compiled to WebAssembly for browser-side calculations.

Features include artifact import and management, single-character and team optimization, damage breakdowns, artifact potential analysis, presets, and custom optimization targets. Results depend on your selected skills, enemy settings, and effect conditions.

[中文](README.md) · [日本語](README_ja.md)

## Local development

Prerequisites: Node.js 22.12+, Rust stable, Git, and a working native Rust build toolchain.

```sh
git clone --recursive https://github.com/Sukwants/artifiction.git genshin_artifact
cd genshin_artifact
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
npm ci
npm run dev
```

`dev` builds WASM, generates metadata, and starts the development server. For subsequent frontend-only changes, use `npm run serve`. Rebuild WASM and metadata after relevant Rust changes. Use `npm run build` to produce `dist/` after generating these prerequisites.

Artifact data, saved sets, and presets are stored per local account in the browser for the current website origin. Export backups before changing browsers or clearing site data. Share links require a separate remote API; direct YAS scanning requires the compatible local scanner service.

The detailed documentation is maintained in Chinese:

- [User guide](src/pages/helps/InstructionPage/instruction.md)
- [YAS web scanning](docs/yas-web.md)
- [Development and deployment](docs/development.md)
- [Architecture](docs/architecture.md)
- [Rust calculation core](mona_docs/src/index.md)
- [Browser automation API](mona_api/mona-api.md)
- [AI contributor instructions](AGENTS.md)
