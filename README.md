# 🏉 Rugby Async Demo - Rust 2024 Edition

[![Rust 2024](https://img.shields.io/badge/Rust-2024%20Edition-orange?logo=rust)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

非同期プログラミングとラグビー戦術の類似性を示す教育的デモプロジェクト

## 📖 概要

このプロジェクトは、Rustの非同期プログラミング（`async`/`await`）の概念を、ラグビーのスタンドオフ（SO）の判断プロセスに例えて説明します。

ラグビーのSOは、ボールを待つ間にディフェンスラインを読み、チームメイトの配置を確認し、複数のオプションを並行して準備します。これはまさに、非同期プログラミングにおける「待ち時間を無駄にしない」という本質と同じです。

## ✨ Rust 2024 Editionの活用

このプロジェクトは、Rust 2024 Editionの新機能とベストプラクティスを活用しています：

- **Prelude改善**: `Future`と`IntoFuture`が自動インポート
- **RPIT (Return Position Impl Trait)**: より簡潔な型シグネチャ
- **Comprehensive Rustdoc**: すべての公開APIに包括的なドキュメント
- **Type Safety**: 明示的なエラー型による型安全性の向上
- **Cargo最適化**: rust-versionフィールドによる依存関係管理

## 🚀 クイックスタート

### 必要要件

- Rust 1.85.0以上（Rust 2024 edition対応）
- Cargo

### インストールと実行

```bash
# リポジトリをclone
git clone https://github.com/nwiizo/2025-rugby-async-demo.git
cd 2025-rugby-async-demo

# メインのデモを実行（基本的な非同期処理）
cargo run

# 高度な例を実行（Rust 2024の機能フル活用）
cargo run --example modern_rugby_2024

# 複雑なゲームシミュレーション（多数の変数を考慮）
cargo run --example complex_game_simulation
```

### 利用可能なサンプル

1. **基本デモ** (`cargo run`):
   - スタンドオフの基本的な判断プロセス
   - 並行処理の基礎

2. **高度な例** (`modern_rugby_2024`):
   - カスタムエラー型
   - 明示的な型定義（Direction, Decision等）
   - Rust 2024のベストプラクティス

3. **複雑なシミュレーション** (`complex_game_simulation`):
   - **10以上の変数を考慮** した現実的な意思決定
   - 試合時間、スコア差、フィールドポジション、天候、疲労度等
   - 7つの主要シナリオ分析
   - リスク評価とバランス判断
```

### 期待される出力

```
=== 攻撃開始 ===

🏉 スクラムハーフからのパスを待機...
👀 ディフェンスラインを読む...
👥 味方のポジショニング確認...
✓ 味方の準備完了
✓ ディフェンス分析完了: 左にギャップあり
✓ ボール受け取り完了
📢 バックスに展開のサイン...
📢 フォワードにサポートのサイン...
✓ フォワード準備完了
✓ バックス準備完了

🧠 状況を統合して判断...

🎯 決定: 左サイドへパス展開
⏱️  判断までの時間: 2.5秒

💡 並行処理により、順次処理の13秒から2.5秒に短縮！
```

## 📚 コード品質

このプロジェクトは、Rustのベストプラクティスに準拠しています：

```bash
# コードフォーマットチェック
cargo fmt -- --check

# Lintチェック（全警告をエラーとして扱う）
cargo clippy -- -D warnings

# テスト実行
cargo test

# ドキュメント生成
cargo doc --no-deps --open
```

## 🎓 学習リソース

このプロジェクトと合わせて読むと理解が深まる資料：

- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
- [The Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## 📝 ライセンス

MIT License - 詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 🤝 コントリビューション

Issue、Pull Request歓迎します！

## 📖 ブログ記事

このプロジェクトの詳しい解説は、以下のブログ記事で読むことができます：

- [非同期プログラミングとラグビー：なぜスタンドオフは優秀なasync関数なのか](TBD)

---

**🏉 "待ち時間を無駄にしない" - それが非同期プログラミングの本質です。**