# backend

Rust API for the EV Investment fund site. Axum + sqlx (Postgres), laid out
along hexagonal / clean-architecture lines.

## Layout

```
src/
├── main.rs            composition root — wires adapters into use cases and serves
├── config.rs          AppConfig from environment
├── domain/            core (pure: no HTTP, no SQL)
│   ├── model/         entities & value objects (User, NewUser)
│   ├── port/          traits the core depends on (UserRepository) — the "ports"
│   └── error.rs       DomainError
├── application/       use cases over the ports (UserService)
├── infrastructure/    driven adapters implementing the ports
│   ├── db.rs          PgPool + migrations
│   └── persistence/   PostgresBlogRepository (sqlx)
└── api/               driving adapter — HTTP (axum)
    ├── router.rs      routes + middleware, mounted under /api/v1
    ├── state.rs       AppState (shared services)
    ├── handler/       request handlers
    ├── dto/           wire types, decoupled from domain models
    └── error.rs       DomainError ▶ HTTP status mapping
```

Dependency direction: `api ─▶ application ─▶ domain ◀─ infrastructure`.
Everything points inward at `domain`; the core depends on nothing outward.

## Develop

```sh
cp .env.example .env        # set DATABASE_URL
cargo run                   # migrations run on startup
```

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
