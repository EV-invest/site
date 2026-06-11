# pc

Internal app for the EV Investment fund — Dioxus 0.7 (web/WASM), laid out with
Feature-Sliced Design mirroring `landing`. Serves on `:3001`.

## Layout

```
src/
├── application/   router.rs (Route + App), layout.rs (sidebar shell)
├── views/         Dashboard, Clients, Investments, NotFound
├── entities/      Fund / Client / Investment UI models
├── features/      feature slices
├── shared/
│   ├── ui/        shadcn-style components (Button, Card, Badge, Input, Table, …)
│   └── lib/       utils — the cn! tailwind-merge class macro
└── lib.rs         module re-exports (pub use App, Route)
input.css            Tailwind entry: imports ../public/tokens.css + @source scans .rs
assets/tailwind.css  generated stylesheet (committed; served by dx)
```

Components are shadcn-style: variant enums with `#[derive(Default)]`/`#[default]`,
`#[props(default)]` on optional props, classes composed with the `cn!` macro.
Depends on the shared `domain` crate; never on `backend`.

## Run only pc

```sh
nix run .#pc                 # → http://localhost:3001
```
This builds Tailwind once (`npm run pc:css`), keeps it rebuilding in the background
(`pc:css:watch`), then runs `dx serve --package pc --port 3001 --interactive false`.

> **Don't run bare `dx serve`** — it serves the static `assets/tailwind.css` but
> does NOT build it, so you'd get stale or empty CSS. Use `nix run .#pc` (or run
> `npm run pc:css:watch` alongside). For API calls to resolve, run the backend too
> (`nix run .#backend` or `.#dev`).

## Tailwind / design tokens

`input.css` imports the shared [`../public/tokens.css`](../public/tokens.css) and
`@source`-scans `src/**/*.rs` for class names. The standalone Tailwind CLI compiles
it to `assets/tailwind.css` — a committed artifact that `dx` serves as-is. Edit the
tokens once; `landing` and `pc` both pick them up.

## wasm-bindgen version pin

`Cargo.toml` pins `wasm-bindgen = "=0.2.118"` to match the `wasm-bindgen-cli` that
nixpkgs' `dioxus-cli` (`dx`) bundles — `dx` refuses to build on a version mismatch.
Bump it in lockstep whenever the pinned `dioxus-cli` changes its CLI version.
