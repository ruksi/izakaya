# 🏮 Ryokan

Ryokan is a template project for running Rust services on [Render](https://render.com/).

- Evaluating if Render is a good replacement for degrading Heroku 😢
- Playing around with `axum` and `sqlx` 🦀

## Features / TODO

- [x] Web Service, works OK but 10 min deploy
- [x] Custom domain with HTTPS, worked very smoothly
- [x] Environment Groups, a nice addition, shared Env Vars / files between services
- [x] PostgreSQL, works perfectly
- [ ] Redis
- [ ] GitHub Actions CI/CD
- [ ] AWS Health Check
- [ ] Using a Docker image vs. Rust build
- [ ] Web Service Job
- [x] Auto Scaling, sounds OK, CPU/Memory based
- [x] Online Shell, works OK but can be slow without an upgrade
- [ ] Web Service SSH
- [ ] Web Service Rollback
- [ ] Web Service Persistent Disk
- [ ] Static Site
- [ ] Cron Job
- [ ] Private Service
- [ ] Background Worker
- [ ] Blueprint
- [ ] `cargo sqlx prepare` 🤔

## Development

> Remember to update both `.rtx.toml` and `rust-toolchain` when changing the Rust version!

```bash
sudo apt update
sudo apt install postgresql-14
cargo install sqlx-cli --features postgres

cd tatami

# configure your environment
cp .env.example .env
vim .env

# create database and apply migrations
cargo sqlx database setup

# apply all pending migrations
#cargo sqlx migrate run

# revert migrations to the given version
#cargo sqlx migrate revert --target-version=0

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

## "Production"

- Login to [Render](https://render.com/)
- Connect your GitHub and give access to this repository if not already

### PostgreSQL

- Create a new Render PostgreSQL
- Configuration:
    - Name: `ryokan-postgresql`
    - Database: `ryokan`
    - User: `ryokan`
    - PostgreSQL Version: `14`

After the database is created, find the "Internal Database URL" from the dashboard.
This is the `DATABASE_URL` you need to set in `tatami`'s `.env`.

### Web Service

`tatami` is an example of a Render Web Service.

- Create a new Render Web Service
- Select this repository as the source
- Configuration:
    - Name: `ryokan-tatami`
    - Branch: `main`
        - You could have a separate branch for production, but I don't think it's necessary
        - Releasing ASAP is a good practice; you should be backwards compatible
        - And if you find issues, you find the sooner and can improve your review and CI process
    - Root Directory: `tatami`
    - Secret Files: `.env`: copy and edit from `./tatami/.env.example`
    - Health Check Path: `/healthz`
    - Pre-Deploy Command:
        - `cargo install sqlx-cli --no-default-features --features native-tls,postgres && cargo sqlx migrate run`
- Go to https://ryokan-tatami.onrender.com/healthz or whatever
- After this, the web service will auto deploy when both:
    - files under the `/tatami` change on the `main` branch
    - health check responds OK after build

If you need to run commands on the server, you can use the web shell.

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
cargo sqlx migrate revert
# but even just that takes _ages_ so you are probably better of taking
# database URL from the dashboard and reverting any migrations locally
```

## Details

We are using a Rust workspace because:

- Allows us to have multiple binaries in the same repository, mono-repo style.
- These binaries can be anything form web services, to CLIs, to libraries, to cron jobs.
- Makes all binaries share the same `Cargo.lock` file and thus dependency versions.
- Binaries are in separate directories for auto deploys.

Render runs or builds the service with `cargo run/build --release`
[by default](https://docs.render.com/deploy-rocket-rust)

Render expects the service to run at 0.0.0.0:10000
[by default](https://docs.render.com/web-services#host-and-port-configuration)

There is less need for developer sites as Render
has [pull request previews](https://docs.render.com/pull-request-previews).
