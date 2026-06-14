# backend

Rust API for the EV Investment fund site. Axum + sqlx (Postgres) +
TigerBeetle (financial ledger), laid out along hexagonal / clean-architecture
lines.

## Layout

The pure core lives outside `backend`:

- **`ev` (external, `architecture` feature)** — generic, I/O-free DDD tactical
  building-block traits (`Identifier`/`Id`, `Entity`, `AggregateRoot`,
  `Repository`, `Reader`, `Gateway`, `UnitOfWork`, `DomainEvent`, `Specification`).
  wasm-safe; lives in [`EV-invest/lib`](https://github.com/EV-invest/lib), pulled in
  as a git dependency and re-exported by `domain` as `domain::architecture`.
- **`domain/`** — the shared domain types (`Blog`, ledger models, value objects,
  `DomainError`) implementing the architecture traits. See [`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md).

`backend` holds the adapters and wiring:

```
src/
├── main.rs            composition root — wires adapters into use cases and serves
├── config.rs          AppConfig from environment
├── domain/
│   └── port/          outbound ports the core depends on (BlogRepository, Ledger)
├── application/       use cases over the ports (BlogService, LedgerService)
├── infrastructure/    driven adapters implementing the ports
│   ├── db.rs          PgPool + migrations
│   ├── persistence/   PostgresBlogRepository (sqlx)
│   └── tigerbeetle/   TigerBeetleLedger (official tigerbeetle crate; a Gateway)
└── api/               driving adapter — HTTP (axum)
    ├── router.rs      routes + middleware, mounted under /api/v1
    ├── state.rs       AppState (shared services)
    ├── handler/       request handlers
    ├── dto/           wire types, decoupled from domain models
    └── error.rs       DomainError ▶ HTTP status mapping
```

Dependency direction: `api ─▶ application ─▶ domain/port ◀─ infrastructure`,
with every crate pointing inward at the pure `domain` types and the `ev` architecture module
(reached via `domain::architecture`). The core depends on nothing outward.

## Run only backend

```sh
nix run .#backend           # → http://localhost:8080
```
Needs a reachable Postgres **and** TigerBeetle — `nix run .#db` `nix run .#tb`
(or `.#dev`, which boots both first). The app migrates Postgres on startup and
connects to TigerBeetle at `TIGERBEETLE_ADDRESS`. Defaults match
[`.env.example`](./.env.example), so a `.env` is optional. Inside the dev shell
you can `cargo run` directly.

```sh
cargo check && cargo clippy -- -D warnings && cargo test
```

The official TigerBeetle Rust client isn't published on crates.io yet (only a
placeholder). A [`.tb-client/`](./.tb-client/) stub mirrors the TB API so the
crate compiles; `nix run .#backend` symlinks a nix-built client with real native
assets before invoking Cargo.

### Endpoints (under `/api/v1`)

| Method | Path          | Description           |
| ------ | ------------- | --------------------- |
| GET    | `/health`     | liveness probe        |
| POST   | `/blogs`      | create a blog post    |
| GET    | `/blogs`      | list blog posts       |
| GET    | `/blogs/{id}` | fetch a blog by id    |

## Extending

Add a feature by following the same flow: model in `domain/model`, a port in
`domain/port`, a use case in `application`, an adapter in `infrastructure`, and
handlers + DTOs in `api`. Wire the new adapter in `main.rs`.

Example — the TigerBeetle integration:

1. `domain/src/model/ledger.rs` — pure domain types (`LedgerAccount`, `LedgerTransfer`, …)
2. `backend/src/domain/port/ledger.rs` — `Ledger` trait (async_trait, object-safe)
3. `backend/src/infrastructure/tigerbeetle/tigerbeetle_ledger.rs` — adapter mapping
   domain ↔ TB types, translating TB errors to `DomainError`
4. `backend/src/application/ledger_service.rs` — `LedgerService` (Arc<dyn Ledger>)
5. `backend/src/main.rs` — wire `TigerBeetleLedger` → `LedgerService` → `AppState`
