# Ryokan 🏮🈺

Ryokan is a research / template project for running Rust services on various PaaS providers.

In essence, this is a mono-repo style Rust project with multiple components that
are configured to auto-deploy on various PaaS providers. ☁️☁️☁️

- Render:
    - a static site: https://futon-render.ryokan.dev/
    - a web app: https://tatami-render.ryokan.dev/
- Railway:
    - a static site: https://futon-railway.ryokan.dev/
    - a web app: https://tatami-railway.ryokan.dev/

> 🚧 These services might go offline later after I've finalized the latency / health testing. 🚧

Using Rust workspaces because:

- Allows us to have multiple binaries in the same repository, mono-repo style.
- These binaries can be anything form web services, to CLIs, to libraries, to cron jobs.
- Makes all binaries share the same `Cargo.lock` file and thus dependency versions.
- Binaries are in separate directories for scoped auto deploys.

## Why?

Mainly to play around with Rust 🦀 but...

I haven't liked the direction Heroku is heading for a while now, so looking for alternatives:

- [x] [Heroku](https://www.heroku.com/)
    - it works, but the free tier is limited and the pricing is just ridiculous
    - no native Rust support, but there is a BuildPack for it
    - no free SSL for the free tier :(
- [x] [Vercel](https://vercel.com/)
    - doesn't really support Rust, and the focus is heavily on frontend
    - could work well if paired with Fly.io; but that's too many moving parts for my liking
- [x] [Dokku](https://dokku.com/)
    - works, but it's not really a managed PaaS if you have to manage it 😅
- [ ] [Render](https://render.com/)
    - intuitive to use and works well in general
    - nice that it basically requires no changes to your repository
    - Rust building for deployment can take 10+ minutes
        - this was on the lowest paid plan
        - there is some build performance boost if you are on a team plan
    - the free PostgreSQL gets deleted after 90 days, which sounds strange
        - effectively, this means you need to pay at least $14/month for an always-on service
        - the PostgreSQL does have a CIDR firewall, which is really nice 🧱
    - the web UI is OK, not the best, but has a lot of options
    - feels better through the web UI than through the CLI
- [ ] [Railway](https://railway.app/)
    - I like the general vibe, like the team rewriting the old Go CLI in Rust
        - you can feel that whoever has made the examples is familiar with Rust
        - happened to read a couple of blog posts and dig the communication style
    - their build system on Nixpacks feels solid; and you can build them locally too
        - [supports a lot of languages out-of-the-box](https://docs.railway.app/reference/nixpacks#supported-languages)
    - builds Rust faster than Render; and seem to automatically cache stuff
        - from `git push` to receiving traffic: Railway 3 min, Render 7 min
        - on Render this was on the cheapest plan, on Railway this was the free tier
    - private network not being initialized pre-deploy is slightly annoying
        - and even in deploy container, it takes a few seconds to initialize 🤷
    - the web UI is real slick and pretty 💅
        - even has a minimalistic PostgreSQL editor if a quick production fix is needed
    - feels just as good to use through the web UI as through the CLI
- [ ] [Fly.io](https://fly.io/)
    - very quick response call times even on the free tier
    - everything feels backend / API focused
    - working through Dockerfiles feels boilerplate-y for simple services
        - Dockerfile-builds are essential for more complex setups 😆
    - you can get a free PostgreSQL / Redis, but from a third party (Supabase / Upstash) 🤔
    - no native GitHub integrations; you have to do custom GitHub Actions
    - the web UI feels a bit clunky
    - feels better through the CLI than through the web UI

## Render

- [x] Environment Groups, a nice addition, shared Env Vars / files between services
- [x] Web Service works OK but 10 min deploy
    - [x] Custom domain with HTTPS, worked very smoothly
    - [x] Online Shell works OK but can be slow without an upgrade
    - [x] Auto Scaling, sounds OK, CPU/Memory based
    - [x] Job, sounds OK, trigger one-off command using `curl`
    - [x] Rollback, works fine, also reverts secrets, etc.
    - [ ] SSH
    - [ ] Persistent Disk
    - [ ] Using a Dockerfile vs. Rust build
- [x] PostgreSQL works perfectly
- [x] Redis
- [x] Static Site, seems straightforward, can build and customize headers / redirects
- [ ] Cron Job
- [ ] Private Service
- [ ] Background Worker
- [ ] Blueprint
- Extra:
    - [ ] GitHub Actions CI/CD
    - [ ] AWS Health Check
    - [ ] `cargo sqlx prepare` 🤔

## Development

> Remember to update both `.rtx.toml` and `rust-toolchain` when changing the Rust version!

### General

```bash
rtx install
```

### 🧘 `tatami` Backend

```bash
cd tatami

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

cargo run
```

### 👘 `kimono` Frontend

```bash
cd kimono
npm install
npm run dev
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

### Details

Render runs or builds the service with `cargo run/build --release`
[by default](https://docs.render.com/deploy-rocket-rust)

Render expects the service to run at 0.0.0.0:10000
[by default](https://docs.render.com/web-services#host-and-port-configuration)

There is less need for developer sites as Render
has [pull request previews](https://docs.render.com/pull-request-previews).
