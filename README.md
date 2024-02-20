# 🏮 Izakaya

**Izakaya** is a playground to answer:

* **Q**: _When to use a Rust web backend instead of e.g. Flask / Django?_
* **Q**: _How different frontends feel against a Rust backend?_
* **Q**: _What different hosting solutions are there for Rust?_

This mono-repo has various backends and frontends, some of which
auto-deploy on the main branch change.

### Backends

* `🟧 tatami`: Rust + Axum + SQLx
  [code](./tatami),
  [online](https://tatami.railway.izakaya.dev/)

### Frontends

* `🍙 onigiri`: Vite + React + React Router + RTK Query
  [code](./onigiri),
  [online](https://onigiri.railway.izakaya.dev/)
* `🍗 yakitori`: Next 14 + App Router + SWR
  [code](./yakitori),
  [online](https://yakitori.railway.izakaya.dev/)
* `🥟 gyoza`: Vite + SvelteKit + TanStack Query
  [code](./gyoza),
  [online](https://gyoza.railway.izakaya.dev/)

## Why?

Mainly an excuse to play with Rust, but...

I love using launch-and-forget Platform as a Service (PaaS) solutions
for personal projects, but I don't like the direction Heroku has been
heading for ages now, so looking for alternatives: [PaaS Provider Notes](./PAAS.md)

For the time being, I'm convinced by [Railway](https://railway.app/).

... and I realized that I don't know the ins and outs of all
these "modern" frontend frameworks, so I decided to try out
a couple of them.

## Development

Each subproject has their own `README` with usage instructions.

I use [mise](https://mise.jdx.dev/) to manage tool/language versions; you can too:

```bash
mise install
```

Or manually set up versions defined in the `.mise.toml` file.

> _Take a seat, read some code and enjoy your time! 🍻_
