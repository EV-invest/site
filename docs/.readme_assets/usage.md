## Layout

| Path | What | Stack | Details |
| ---- | ---- | ----- | ------- |
| [`landing/`](./landing) | public marketing site | Next.js 16 · FSD · Tailwind · TS | [README](./landing/README.md) |
| [`backend/`](./backend) | HTTP API | Rust · Axum · sqlx (Postgres) | [README](./backend/README.md) |
| [`cabinet/`](./cabinet) | internal app (web/WASM) | Rust · Dioxus 0.7 · FSD | [README](./cabinet/README.md) |
| [`domain/`](./domain) | shared domain types (pure, no I/O) | Rust | — |
| [`public/tokens.css`](./public/tokens.css) | design tokens | CSS (Tailwind v4) | — |

`domain` is the shared source of truth for types — `backend` and `cabinet` depend on it,
never on each other. `public/tokens.css` is the shared design source of truth for
`landing` and `cabinet` (each wires Tailwind its own way — see their READMEs).

## Run

Every app is a flake app. `nix run` resolves the repo root at runtime, so there's
no need to enter the dev shell first.

| Command | Brings up | Port |
| ------- | --------- | ---- |
| `nix run .#dev` | everything: Postgres → backend → landing → cabinet | — |
| `nix run .#landing` | landing only | 3000 |
| `nix run .#backend` | backend only (needs a DB — `.#db` or `.#dev`) | 8080 |
| `nix run .#cabinet` | cabinet only (Tailwind watch + `dx serve`) | 8081 |
| `nix run .#db` | local Postgres (cluster under `.pg/`, trust auth) | 5432 |
| `nix run .#tb` | local TigerBeetle (data under `.tb/`, single replica) | 3001 |

`.#dev` starts Postgres first and waits for it before launching the backend (which
migrates on boot); one Ctrl-C tears the whole stack down. Per-app build, test, and
layout details live in each folder's README — see the table above.

A dev shell with the full toolchain (Rust nightly + `wasm32`, Node, Postgres,
Playwright browsers) is auto-activated by `.envrc` + direnv, or via `nix develop`.

> **Production** (`.#prod`) is intentionally not wired up yet — the Docker-vs-Nix
> packaging decision is still open.
