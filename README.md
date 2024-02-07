# 🏮 Izakaya

**Izakaya** is an ongoing learning project to answer:

* **Q**: _When to use a Rust web backend instead of e.g. Flask / Django?_
* **Q**: _How frontend frameworks feel against a Rust backend?_
* **Q**: _What different hosting solutions are there for Rust?_

This mono-repo has various backends and frontends, some of which
auto-deploy on the main branch change.

### Backends

* 🟧 `tatami`: Rust + Axum + SQLx

### Frontends

* 🍙 `onigiri`: Vite + React + React Router
* 🍗 `yakitori`: Next 14 + App Router

## Why?

Mainly an excuse to work with Rust, but...

I love using launch-and-forget Platform as a Service (PaaS) solutions
for personal projects, but I don't like the direction Heroku has been
heading for a while now, so looking for alternatives: [PaaS Provider Notes](./PAAS.md)

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

> _Take a seat and enjoy your time! 🍻_
