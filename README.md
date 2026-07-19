# Habit Tracker

継続したい習慣を登録し、日々の実行記録を管理するアプリケーションです。

現在は、RustバックエンドとPostgreSQL 18のローカル開発環境を準備した段階です。詳細な構想は [grand-design.md](./grand-design.md) を参照してください。

## 技術スタック

- バックエンド: Rust、Actix Web、SQLx
- データベース: PostgreSQL 18
- 認証: JWT
- フロントエンド: Angular（予定）

## 必要なツール

- Rust stable / nightly
- cargo-watch
- Docker / Docker Compose
- just
- direnv（任意）

```bash
rustup toolchain install nightly --component rustfmt
cargo install cargo-watch
```

## 環境変数

リポジトリ直下に、次の内容を参考に `.envrc` を作成してください。`.envrc` はGit管理から除外されています。

```bash
export DATABASE_HOST=127.0.0.1
export DATABASE_PORT=5432
export DATABASE_USER=habit_tracker
export DATABASE_PASSWORD=change-me
export DATABASE_NAME=habit_tracker
export DATABASE_SSLMODE=disable
export DATABASE_URL="postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}?sslmode=${DATABASE_SSLMODE}"

export SERVER_HOST=127.0.0.1
export SERVER_PORT=8000

export PGADMIN_EMAIL=admin@example.com
export PGADMIN_PASSWORD=change-me
```

direnvを使用する場合は、ファイル作成後に読み込みを許可します。

```bash
direnv allow
```

direnvを使用しない場合は、バックエンドやDocker Composeを実行するシェルへ同じ環境変数を設定してください。

## 開発環境

利用可能なコマンドは次の操作で確認できます。

```bash
just
```

PostgreSQLとpgAdminを起動します。

```bash
just dup
```

- PostgreSQL: `127.0.0.1:5432`
- pgAdmin: <http://127.0.0.1:8080>

バックエンドを実行します。

```bash
just run
```

起動後、ヘルスチェックを利用できます。

```bash
curl http://127.0.0.1:8000/health
```

```json
{"status":"ok"}
```

ソースコードの変更を監視してバックエンドを再実行します。

```bash
just watch
```

Nightly版rustfmtでコードを整形します。

```bash
just fmt
```

PostgreSQLとpgAdminを停止します。

```bash
just dwn
```

## ディレクトリ構成

```text
.
├── backend/         Rustバックエンド
├── docker/          PostgreSQLとpgAdminの開発環境
├── grand-design.md  アプリケーション構想
└── justfile         開発用コマンド
```
