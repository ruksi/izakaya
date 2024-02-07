# PaaS Provider Notes

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
    - **the free PostgreSQL gets deleted after 90 days**, which sounds strange
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
        - and even in deploy-container, it takes a few seconds to initialize 🤷
    - the web UI is real slick and pretty 💅
        - even has a minimalistic database editor if a quick production fix is needed
    - feels just as good to use through the web UI as through the CLI
- [ ] [Fly.io](https://fly.io/)
    - rapid response call times even on the free tier
    - everything feels backend / API focused
    - working through Dockerfiles feels boilerplate-y for simple services
        - Dockerfile-based builds are essential for more complex setups anyway
    - you can get a free PostgreSQL / Redis, but from a third party (Supabase / Upstash) 🤔
    - no native GitHub integrations; you have to do custom GitHub Actions
    - the web UI feels a bit clunky
    - feels better through the CLI than through the web UI
    - don't like the general vibe, but probably worth a second look later

> With all of these PaaS providers; I would still recommend using a managed PostgreSQL from a cloud
> provider for production. It's just cheaper, and if the region is the same, you won't notice the
> difference.
