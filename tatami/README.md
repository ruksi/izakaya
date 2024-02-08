# `🟧 tatami` Backend

Rust backend with Axum and SQLx.

## Development

```bash
sudo apt update
sudo apt install postgresql-14
cp .env.example .env
vim .env
cargo install sqlx-cli --features postgres
cargo sqlx database setup
cargo run
```

__SQLx should be run in offline mode in production as the server itself runs migrations.__
But this causes SQLx to crash the build as the database is not yet in sync.
An approach we use is to set `SQLX_OFFLINE=true` on production and CI.

```bash
# you should run these only in development:

# check that SQLx offline statements are in sync with the database
cargo sqlx prepare --check

# generate the SQLx offline statements
cargo sqlx prepare

# if the offline statements are out of sync, the server won't build in production / CI
```

Database management:

```bash
# list migrations
cargo sqlx migrate info
# 1/installed uuid extension
# ...
# 5/pending   user

# apply all pending migrations
cargo sqlx migrate run

# apply/revert to the given migration
cargo sqlx migrate revert --target-version=0
```

To create a new migration:

```bash
cargo sqlx migrate add -r my_migration
vim migrations/*_my_migration.up.sql
vim migrations/*_my_migration.down.sql

# repeat the following until satisfied
cargo sqlx migrate run
cargo sqlx migrate revert
```

Run these from time to time:

```bash
cargo clippy --fix
cargo fmt
```

## "Production"

Requirements:

* a [Railway](https://railway.app/) project
* a PostgreSQL service in that project e.g. `database`
* a Redis service in that project e.g. `cache`
* this repository or a fork linked to the Railway project

Steps:

* Add a new service from "GitHub Repository"
    * Select this repository.
    * Variables:
        * `RUST_LOG`: `tatami=debug,tower_http=debug`
        * `DATABASE_URL`: `${{database.DATABASE_URL}}`
        * `CACHE_URL`: `${{cache.REDIS_URL}}`
        * `SECRET_KEY`: generate a random 64 character string
        * `SQLX_OFFLINE`: `true`
    * Settings:
        * Service Name: `🟧 tatami`
        * Railway Config File: `tatami/railway.toml`
    * Click "Deploy"

Next Steps:

* Configure public networking for `🟧 tatami` after it runs.
* Edit `FRONTEND_URL` to include frontends you launch, separated by commas e.g.
    * `http://localhost:3000,http://localhost:5173`
