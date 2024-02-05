# Izakaya on Render

TODO:

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
    - [x] AWS Health Check
    - [x] `cargo sqlx prepare` 🤔

## "Production"

- Login to [Render](https://render.com/)
- Connect your GitHub and give access to this repository if not already

### Database

- Create a new Render PostgreSQL
- Configuration:
    - Name: `izakaya-database`
    - Database: `izakaya`
    - User: `izakaya`
    - PostgreSQL Version: `14`

After the database is created, find the "Internal Database URL" from the dashboard.
This is the `DATABASE_URL` you need to set in `tatami`'s `.env`.

### Cache

- Create a new Render Redis
- Configuration:
    - Name: `izakaya-cache`

After the store is created, find the "Internal Redis URL" from the dashboard.
This is the `CACHE_URL` you need to set in `tatami`'s `.env`.

### Backend

- Create a new Render Web Service
- Select this repository as the source
- Configuration:
    - Name: `izakaya-tatami`
    - Branch: `main`
        - You could have a separate branch for production, but I don't think it's necessary
        - Releasing ASAP is a good practice; you should be backwards compatible
        - And if you find issues, you find the sooner and can improve your review and CI process
    - Root Directory: `tatami`
    - Secret Files: `.env`: copy and edit from `./tatami/.env.example`
    - Health Check Path: `/healthz`
- Check https://izakaya-tatami.onrender.com/healthz or whatever it is
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

This is painfully slow though, so you are probably better off running migrations locally.

## Notes

Render runs or builds the service with `cargo run/build --release`
[by default](https://docs.render.com/deploy-rocket-rust)

Render expects the service to run at 0.0.0.0:10000
[by default](https://docs.render.com/web-services#host-and-port-configuration)

There is less need for developer sites as Render
has [pull request previews](https://docs.render.com/pull-request-previews).
