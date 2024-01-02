# 🏮🈺 Ryokan

Ryokan is a research / template project for running Rust services on various PaaS providers.

## Why?

Mainly to play around with Rust 🦀

I haven't liked the direction Heroku is heading for a while now so looking for alternatives:

- [x] [Heroku](https://www.heroku.com/)
    - works, but the free tier is limited and the pricing is just ridiculous
    - no native Rust support but there is a BuildPack for it
    - no free SSL for the free tier :(
- [x] [Vercel](https://vercel.com/)
    - doesn't really support Rust, and the focus is heavily on frontend
- [x] [Dokku](https://dokku.com/)
    - works, but it's not really a PaaS if you have to manage it, even a little
- [ ] [Render](https://render.com/)
    - very intuitive to use and works well in general
    - Rust building for deploy takes 10 minutes if you don't pay for extra build power...
    - nice that it requires basically no changes to your repository
    - the free PostgreSQL gets deleted after 90 days, which sounds strange
    - the web UI is OK, but not great, but has a lot of options
    - feels better through the web UI than through the CLI
- [ ] [Railway](https://railway.app/)
    - I like the general vibe; like the team rewriting the old Go CLI in Rust
    - their build system on Nixpacks feels solid; and you can build them locally too
    - because of
      this, [supports a lot of languages out-of-the-box](https://docs.railway.app/reference/nixpacks#supported-languages)
    - builds are a lot faster than with Render; and seem to automatically cache stuff
    - the web UI is real slick and pretty 💅
    - feels just as good to use through the web UI as through the CLI
- [ ] [Fly.io](https://fly.io/)
    - very quick response times even on the free tier
    - everything feels... backend / API focused
    - although I work a lot with Docker, it feels too boilerplate-y for a simple services
    - you can get free PostgreSQL and Redis, but both come from a 3rd party (Supabase / Upstash)
    - no native GitHub integrations; you have to do custom GitHub Actions
    - the web UI feels horrible
    - feels better through the CLI than through the web UI

## Render

- [x] Web Service, works OK but 10 min deploy
    - [x] Custom domain with HTTPS, worked very smoothly
    - [x] Online Shell, works OK but can be slow without an upgrade
    - [x] Auto Scaling, sounds OK, CPU/Memory based
    - [x] Job, sounds OK, trigger one-off commands using `curl`
    - [x] Rollback, works fine, also reverts secrets etc.
    - [ ] SSH
    - [ ] Persistent Disk
- [x] Environment Groups, a nice addition, shared Env Vars / files between services
- [x] PostgreSQL, works perfectly
- [x] Redis
- [ ] GitHub Actions CI/CD
- [ ] AWS Health Check
- [ ] Using a Docker image vs. Rust build
- [x] Static Site, seems straightforward, can build and customize headers / redirects
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
    - Name: `ryokan-database`
    - Database: `ryokan`
    - User: `ryokan`
    - PostgreSQL Version: `14`

After the database is created, find the "Internal Database URL" from the dashboard.
This is the `DATABASE_URL` you need to set in `tatami`'s `.env`.

### Redis

- Create a new Render Redis
- Configuration:
    - Name: `ryokan-cache`

After the store is created, find the "Internal Redis URL" from the dashboard.
This is the `CACHE_URL` you need to set in `tatami`'s `.env`.

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
        - `cargo install sqlx-cli --no-default-features --features postgres && cargo sqlx migrate run`
- Go to https://ryokan-tatami.onrender.com/healthz or whatever
- After this, the web service will auto deploy when both:
    - files under the `/tatami` change on the `main` branch
    - health check responds OK after build

If you need to run commands on the server, you can use the web shell.

```bash
cargo install sqlx-cli --no-default-features --features postgres
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
