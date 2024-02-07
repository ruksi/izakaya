# 🟧 `tatami` Backend

Rust backend using Axum and SQLx.

```bash
sudo apt update
sudo apt install postgresql-14
cargo install sqlx-cli --features postgres

# configure your environment
cp .env.example .env
vim .env

# create database and apply migrations
cargo sqlx database setup

# apply all pending migrations
#cargo sqlx migrate run

# revert migrations to the given version
#cargo sqlx migrate revert --target-version=0

# sqlx runs in offline mode in production as the app itself 
# runs migrations, but sqlx crashes the build as the database 
# is not yet in sync... the same issue with CI, so set 
# `SQLX_OFFLINE=true` on both

# locally, to check your queries are in sync with the database
cargo sqlx prepare --check --workspace
# if not, regenerate them to `.sqlx/` and push to version control
cargo sqlx prepare --workspace

cargo run
```

If you need to change the database:

```bash
cargo sqlx migrate add -r my_migration
# edit the files...
cargo sqlx migrate run
cargo sqlx migrate revert
cargo sqlx migrate run
```

Run these from time to time:

```bash
cargo clippy --fix
cargo fmt
```
