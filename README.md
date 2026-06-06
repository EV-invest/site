# ev_site
![Minimum Supported Rust Version](https://img.shields.io/badge/nightly-1.92+-ab6000.svg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ev_site.svg?color=fc8d62&logo=rust" height="20" style=flat-square>](https://crates.io/crates/ev_site)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs&style=flat-square" height="20">](https://docs.rs/ev_site)
![Lines Of Code](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/ev_site-loc.json)
<br>
[<img alt="ci errors" src="https://img.shields.io/github/actions/workflow/status/valeratrades/ev_site/errors.yml?branch=master&style=for-the-badge&style=flat-square&label=errors&labelColor=420d09" height="20">](https://github.com/valeratrades/ev_site/actions?query=branch%3Amaster)
[<img alt="ci warnings" src="https://img.shields.io/github/actions/workflow/status/valeratrades/ev_site/warnings.yml?branch=master&style=for-the-badge&style=flat-square&label=warnings&labelColor=d16002" height="20">](https://github.com/valeratrades/ev_site/actions?query=branch%3Amaster)

Site of **EV Investment** fund.

## Stack

| Layer | Tech | Path | Port |
|-------|------|------|------|
| Landing | Next.js 16 · App Router · FSD · Tailwind | `landing/` | :3000 |
| CRM | Dioxus 0.7 · WASM · FSD · Tailwind | `crm/` | :3001 |
| API | Rust · Axum · Hexagonal architecture | `backend/` | :8080 |
| Domain | Shared Rust types · no I/O | `domain/` | — |
| DB | PostgreSQL (local, managed by Nix) | `.pg/` | :5432 |

---

## Dev

### Prerequisites

- **Nix** with flakes enabled
- **direnv** — run once from the repo root:

```sh
direnv allow
```

This activates the devshell on every subsequent `cd` into the repo (Rust nightly, Node, `DYLD_LIBRARY_PATH` for the wasm32 linker on macOS).

- **Backend env** — copy and adjust once:

```sh
cp backend/.env.example backend/.env
```

- **Landing deps** — install once:

```sh
npm --prefix landing install
```

### Running all services

Four terminals from the repo root:

```sh
# 1 — PostgreSQL  (cluster in .pg/, created on first run)
nix run .#db

# 2 — Backend API
cargo run -p backend

# 3 — Landing
npm run dev

# 4 — CRM  (Tailwind watcher + Dioxus dev server)
npm run crm:css:watch &
dx serve --package crm --port 3001
```

### URLs

| Service | URL |
|---------|-----|
| Landing | http://localhost:3000 |
| Backend / Swagger UI | http://localhost:8080 · http://localhost:8080/swagger-ui |
| CRM | http://localhost:3001 |

---

## Design tokens

`public/tokens.css` is the **single source of truth** for the design system — both apps import it:

- **Landing** — consumed via `landing/application/styles/globals.css`, processed by Next.js + Tailwind v4.
- **CRM** — built separately: `npm run crm:css` → `crm/assets/tailwind.css` (committed; re-run when tokens change).

The Figma file ([`Main`](https://www.figma.com/design/e0V2P1cQpEFRuXTeNtEMh6/Main?node-id=10-2)) is **code-first**: `public/tokens.css` is authoritative, Figma conforms to it.

---

## Checks

```sh
cargo clippy --workspace -- -D warnings
cargo test --workspace
npm run lint && npm run typecheck     # delegates to landing/
```

---

<br>

<sup>
This repository follows <a href="https://github.com/valeratrades/.github/tree/master/best_practices">my best practices</a>. For architecture, see <a href="./docs/ARCHITECTURE.md">ARCHITECTURE.md</a>.
</sup>

#### License

<sup>
Licensed under <a href="LICENSE">Blue Oak 1.0.0</a>
</sup>
