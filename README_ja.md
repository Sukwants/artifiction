<!-- 文書整理：Codex -->
# 叶师傅开锁铺（Artifiction）

原神のダメージ計算・聖遺物最適化ツールです。Vue 3 の画面と、WebAssembly に変換した Rust の計算処理を使用します。

聖遺物のインポートと管理、単独キャラクター・チームの最適化、ダメージ内訳、聖遺物の強化期待値分析、計算プリセットなどに対応しています。計算結果はスキル、敵、効果の条件設定によって変わります。

[中文](README.md) · [English](README_en.md)

## ローカル開発

Node.js 22.12 以上、Rust stable、Git、および Rust 用のネイティブビルド環境が必要です。

```sh
git clone --recursive https://github.com/Sukwants/artifiction.git genshin_artifact
cd genshin_artifact
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
npm ci
npm run dev
```

`dev` は WASM のビルド、メタデータの生成、開発サーバーの起動を順に実行します。フロントエンドだけを変更する場合は、次回から `npm run serve` を使用できます。Rust を変更した場合は、対応する WASM やメタデータを再生成してください。

聖遺物・保存したセット・プリセットは、現在のサイトのブラウザー内にローカルアカウントごとに保存されます。ブラウザーの変更やサイトデータの削除前に、ファイルとしてバックアップしてください。共有リンクには別の API、YAS の直接スキャンには対応するローカルプログラムが必要です。

詳細な文書は中国語で管理しています。

- [ユーザーガイド](src/pages/helps/InstructionPage/instruction.md)
- [YAS のウェブ接続](docs/yas-web.md)
- [開発とデプロイ](docs/development.md)
- [プロジェクト構成](docs/architecture.md)
- [計算コアの開発](mona_docs/src/index.md)
- [AI 向けの作業指針](AGENTS.md)
