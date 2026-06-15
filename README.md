# ev_site
![Minimum Supported Rust Version](https://img.shields.io/badge/nightly-1.92+-ab6000.svg)
![Lines Of Code](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/ev_site-loc.json)
<br>
[<img alt="ci errors" src="https://img.shields.io/github/actions/workflow/status/EV-invest/site/errors.yml?branch=main&style=for-the-badge&style=flat-square&label=errors&labelColor=420d09" height="20">](https://github.com/EV-invest/site/actions?query=branch%3Amain) <!--NB: Won't find it if repo is private-->
[<img alt="ci warnings" src="https://img.shields.io/github/actions/workflow/status/EV-invest/site/warnings.yml?branch=main&style=for-the-badge&style=flat-square&label=warnings&labelColor=d16002" height="20">](https://github.com/EV-invest/site/actions?query=branch%3Amain) <!--NB: Won't find it if repo is private-->

site of `EV Investment` fund — a monorepo of three apps over a shared Rust
`domain` crate and a single design-token file.

## Usage
### Layout

| Path | What | Stack | Details |
| ---- | ---- | ----- | ------- |
| [`landing/`](./docs/.readme_assets/landing) | public marketing site | Next.js 16 · FSD · Tailwind · TS | [README](./docs/.readme_assets/landing/README.md) |
| [`backend/`](./docs/.readme_assets/backend) | HTTP API | Rust · Axum · sqlx (Postgres) | [README](./docs/.readme_assets/backend/README.md) |
| [`cabinet/`](./docs/.readme_assets/cabinet) | internal app (web/WASM) | Rust · Dioxus 0.7 · FSD | [README](./docs/.readme_assets/cabinet/README.md) |
| [`domain/`](./docs/.readme_assets/domain) | shared domain types (pure, no I/O) | Rust | — |
| [`public/tokens.css`](./docs/.readme_assets/public/tokens.css) | design tokens | CSS (Tailwind v4) | — |

`domain` is the shared source of truth for types — `backend` and `cabinet` depend on it,
never on each other. `public/tokens.css` is the shared design source of truth for
`landing` and `cabinet` (each wires Tailwind its own way — see their READMEs).

### Run

Every app is a flake app. `nix run` resolves the repo root at runtime, so there's
no need to enter the dev shell first.

| Command | Brings up | Port |
| ------- | --------- | ---- |
| `nix run .#dev` | everything: Postgres → backend → landing → cabinet | — |
| `nix run .#landing` | landing only | 3000 |
| `nix run .#backend` | backend only (needs a DB — `.#db` or `.#dev`) | 8080 |
| `nix run .#cabinet` | cabinet only (Tailwind watch + `dx serve`) | 3001 |
| `nix run .#db` | local Postgres (cluster under `.pg/`, trust auth) | 5432 |
| `nix run .#tb` | local TigerBeetle (data under `.tb/`, single replica) | 3001 |

`.#dev` starts Postgres first and waits for it before launching the backend (which
migrates on boot); one Ctrl-C tears the whole stack down. Per-app build, test, and
layout details live in each folder's README — see the table above.

A dev shell with the full toolchain (Rust nightly + `wasm32`, Node, Postgres,
Playwright browsers) is auto-activated by `.envrc` + direnv, or via `nix develop`.

> **Production** (`.#prod`) is intentionally not wired up yet — the Docker-vs-Nix
> packaging decision is still open.

<!-- Per-app details live in each folder's README (landing/, backend/, cabinet/) — not duplicated here. -->


<br>

<sup>
	This repository follows <a href="https://github.com/valeratrades/.github/tree/master/best_practices">my best practices</a> and <a href="https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md">Tiger Style</a> (except "proper capitalization for acronyms": (VsrState, not VSRState) and formatting). For project's architecture, see <a href="./docs/ARCHITECTURE.md">ARCHITECTURE.md</a>.
</sup>

#### License

<sup>
	Licensed under <a href="LICENSE">Blue Oak 1.0.0</a>
</sup>

<br>

<sub>
	Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be licensed as above, without any additional terms or conditions.
</sub>

