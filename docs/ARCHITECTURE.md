# Architecture

The repo is a Cargo workspace (plus the TypeScript `landing` app). The Rust side
is layered along hexagonal / clean-architecture lines over a shared set of DDD
tactical building blocks.

## Crate graph

```
ev (architecture feature) ◀── domain ◀── backend
                                  ▲
                                  └─────── cabinet (Dioxus/WASM)
```

- **`ev` (external, `architecture` feature)** — generic, I/O-free DDD tactical
  building blocks. Lives in the shared [`EV-invest/lib`](https://github.com/EV-invest/lib)
  repo and is pulled in as a git dependency. No runtime deps, so it is wasm-safe;
  `domain` re-exports it as `domain::architecture`, so `backend`/`cabinet` never depend on it
  directly. Defines the vocabulary; owns no concrete domain types.
- **`domain/`** — the shared source of truth for domain types (blog + ledger),
  implementing the architecture traits. Pure; compiles to native **and** wasm.
- **`backend/`** — Axum + sqlx (Postgres) + TigerBeetle; implements the ports.
- **`cabinet/`** — Dioxus/WASM client; depends on `domain` only.

## Architecture building blocks (`domain::architecture`)

| Trait / type | Role | Object-safe? |
| --- | --- | --- |
| `Id<Tag, U>` / `Identifier` | typed identity newtype — a `TransferId` can't be passed where an `AccountId` is expected | no (always concrete) |
| `Entity` / `AggregateRoot` | stable identity; the transactional consistency boundary | no |
| `Repository` / `Reader` | marker super-traits tying a port to one aggregate (CRUD lives on the leaf trait) | yes |
| `Gateway` | anti-corruption boundary to an external transactional system | yes |
| `UnitOfWork` | one atomic Postgres transaction; `commit`/`rollback` consume it | yes |
| `DomainEvent` / `EmitsEvents` | past-tense facts; defined, not yet wired (future outbox) | no |
| `Specification<T>` | composable in-memory predicate | yes (core method) |

Value objects use **parse-don't-validate** (`Title`, `Slug`, `Body`, `Amount`,
`LedgerCode`, `Code`): a private field plus a fallible `parse`/`new` constructor.
There is deliberately **no** `ValueObject` trait — it would enforce nothing the
private field and constructor don't already guarantee.

## Consistency boundaries — Repository vs Gateway

Postgres and TigerBeetle are two independent consistency boundaries:

- A `UnitOfWork` is exactly one sqlx Postgres transaction; repositories enroll in it.
  The architecture trait is in place, but no concrete Postgres adapter exists yet: every
  current use case is a single statement on the pool, so the adapter is deferred
  to the first write that spans more than one aggregate (e.g. a blog plus an outbox row).
- TigerBeetle is a `Gateway`, never a `Repository`, and is **unreachable** from a
  `UnitOfWork` (there is no `UnitOfWork::gateway()`), so the type system forbids
  enrolling it in a Postgres transaction. It owns its own identity, invariants,
  and atomicity.

Any operation spanning both stores is an explicit application-layer saga, not an
atomic transaction. When that need arrives, the intended mechanism is a domain
event written to a Postgres outbox **inside the same `UnitOfWork`** as the state
change — which is why `DomainEvent` exists but is unwired today.
