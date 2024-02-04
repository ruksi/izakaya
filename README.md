# 🏮 Ryokan

Ryokan is a research and learning project for running Rust services on various PaaS providers.

This is a mono-repo with many big components (e.g., frontend, backend) that
are configured to auto-deploy to different PaaS providers.

> 🚧 These deployments might go offline at any time. 🚧

- Render:
    - backend: https://tatami-render.ryokan.dev/
    - web app: -
    - [setup instructions](./RENDER.md)
- Railway:
    - backend: https://tatami-railway.ryokan.dev/
    - web app: -

## Why?

Mainly to play around with Rust 😊 but...

I don't like the direction Heroku has been heading, so looking for alternatives:

- [x] [Heroku](https://www.heroku.com/)
    - no native Rust support, but there is a BuildPack for it
    - no free SSL for the free tier :(
    - free tier is limited and the pricing is just ridiculous
    - works, but hurdles with the free tier, and you will migrate away sooner than later
- [x] [Vercel](https://vercel.com/)
    - doesn't really support Rust, and the focus is heavily on frontend
    - could work if paired with Fly.io; but that's too many moving parts for my liking
- [x] [Dokku](https://dokku.com/)
    - works, but you have to manage it yourself; I could but don't want to
- [x] [Render](https://render.com/)
    - intuitive to use and works well in general
    - nice that it basically requires no changes to your repository
    - Rust building for deployment can take 10+ minutes
        - this was on the lowest _paid_ plan
        - there is some build performance boost if you are on a team plan
    - the free PostgreSQL gets deleted after 90 days, which sounds strange
        - effectively, this means you need to pay at least $14/month for an always-on service
        - the PostgreSQL does have a CIDR firewall, which is really nice 🧱
    - the web UI is OK, not the best, but has a lot of options
    - feels better through the web UI than through the CLI
- [x] [Railway](https://railway.app/)
    - I like the general vibe
        - you can feel that whoever has made the examples is familiar with Rust
        - happened to read a couple of blog posts and I like the openness
    - their build system on Nixpacks feels solid; and you can build them locally too
        - [supports a lot of languages out-of-the-box](https://docs.railway.app/reference/nixpacks#supported-languages)
    - Rust builds are fast; and seem to automatically utilize caches
        - from `git push` to receiving traffic: Railway 3 min, Render 7 min
        - on Render this was on the cheapest plan, on Railway this was the free tier
    - private network not being initialized pre-deploy is slightly annoying
        - and even in deploy container, it takes a few seconds to initialize 🤷
    - the web UI is real slick and pretty 💅
        - even has a minimalistic database editor if a quick production fix is needed
    - feels just as good to use through the web UI as through the CLI
- [ ] [Fly.io](https://fly.io/)
    - rapid response call times even on the free tier
    - everything feels backend / API focused
    - working through Dockerfiles feels boilerplate-y for simple services
        - Dockerfile-builds are essential for more complex setups anyway
    - you can get a free PostgreSQL / Redis, but from a third party (Supabase / Upstash) 🤔
    - no native GitHub integrations; you have to do custom GitHub Actions
    - the web UI feels a bit clunky
    - feels better through the CLI than through the web UI
    - don't like the general vibe, but probably worth a second look later

> With all of these PaaS providers; I would still recommend using a managed PostgreSQL from a cloud
> provider for production.
> It's just cheaper, and if the region is the same, you won't notice the difference.

## Development

Ryokan uses Rust workspaces:

- Allows us to have all deployables in the same repository.
- Easier to add custom Rust macro crates later.
- Easier to add additional service and worker binaries, etc. later.
- Makes all binaries share the same `Cargo.lock` file and thus dependency versions.
- Deployables go to separate directories for directory scoped auto deploys.

> Remember to update both `.rtx.toml` and `rust-toolchain` when changing the Rust version!

### General

Install the correct versions of the required tools:

```bash
rtx install # TODO: upgrade to `mise`
```

Run these from time to time:

```bash
cargo clippy --fix
cargo fmt
```

### 🛏️ `tatami` Backend

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

### 👘 `kimono` Frontend

```bash
cd kimono
npm install
npm run dev
```
