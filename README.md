# 🏮 Ryokan

Ryokan is a template project for running Rust services on [Render](https://render.com/).

Trying to figure out if Render is a good replacement for degraded Heroku. 😢

## Features / TODO

- [x] Render Web Service
- [x] Custom domain with HTTPS
- [ ] Render PostgreSQL
- [ ] Render Redis
- [ ] GitHub Actions CI/CD
- [ ] AWS Health Check
- [ ] Render Jobs
- [ ] Render Auto Scaling
- [x] Render Web Shell
- [ ] Render Rollbacks
- [ ] Render Persistent Disks
- [ ] Render Static Site
- [ ] Render Cron Job
- [ ] Render Private Service
- [ ] Render Blueprints

## Development

> Remember to update both `.rtx.toml` and `rust-toolchain` when changing the Rust version!

```bash
cargo run
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

After the database is created, find the "Internal Database URL" from the dashboard.

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
    - Environment Variables:
        - `PORT=10000`
    - Secret Files:
        - `.env`: copy and edit from `./tatami/.env.dev`
    - Advanced > Health Check Path: `/healthz`
- Go to https://ryokan-tatami.onrender.com/healthz or whatever
- After this, the web service will auto deploy when both:
    - files under the `/tatami` change on the `main` branch
    - health check responds OK after build

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
