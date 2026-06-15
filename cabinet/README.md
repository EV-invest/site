# cabinet

Internal app for the EV Investment fund — Dioxus 0.7 (web/WASM), laid out with
Feature-Sliced Design mirroring `landing`. Serves on `:3001`.

## Layout

Module folders live at the crate root (no `src/`), mirroring `landing`'s
FSD layout. `Cargo.toml` points `[lib]`/`[[bin]]` at `lib.rs`/`main.rs`.

```
application/   router.rs (Route + App), layout.rs (sidebar shell)
views/         Dashboard, Clients, Investments, NotFound
entities/      Fund / Client / Investment UI models
features/      feature slices (error_monitoring = Sentry)
shared/
├── ui/        shadcn-style components (Button, Card, Badge, Input, Table, …)
└── lib/       utils — the cn! tailwind-merge class macro
lib.rs         module re-exports (pub use App, Route)
main.rs        entry: `dx serve` shim (native) + `dioxus::launch` (wasm)
input.css            Tailwind entry: imports ../public/tokens.css + @source scans .rs
assets/tailwind.css  generated stylesheet (committed; served by dx)
```

Components are shadcn-style: variant enums with `#[derive(Default)]`/`#[default]`,
`#[props(default)]` on optional props, classes composed with the `cn!` macro.
Depends on the shared `domain` crate; never on `backend`.

## Run only cabinet

```sh
nix run .#cabinet                 # → http://localhost:3001
```
This builds Tailwind once (`npm run cabinet:css`), keeps it rebuilding in the background
(`cabinet:css:watch`), then runs `dx serve --package cabinet --port 3001 --interactive false`.

> **Don't run bare `dx serve`** — it serves the static `assets/tailwind.css` but
> does NOT build it, so you'd get stale or empty CSS. Use `nix run .#cabinet` (or run
> `npm run cabinet:css:watch` alongside). For API calls to resolve, run the backend too
> (`nix run .#backend` or `.#dev`).

## Tailwind / design tokens

`input.css` imports the shared [`../public/tokens.css`](../public/tokens.css) and
`@source`-scans `**/*.rs` for class names. The standalone Tailwind CLI compiles
it to `assets/tailwind.css` — a committed artifact that `dx` serves as-is. Edit the
tokens once; `landing` and `cabinet` both pick them up.

## Error monitoring (Sentry)

`features/error_monitoring/` is the single entry point, mirroring landing's
`features/error-monitoring`. The native `sentry` Rust crate has no browser-wasm
transport, so the Sentry **JavaScript Browser SDK** (`bundle.tracing.min.js` —
errors + light tracing, **no Session Replay**) is loaded from CDN and driven
from Rust through a thin wasm-bindgen bridge:

- `init()` — called once from the `App` component (`use_hook`); injects the SDK
  and installs a panic hook forwarding Rust panics to Sentry. It runs on first
  render rather than from `main` so it lands *after* Dioxus installs its own
  (debug-only) panic hook and can chain over it instead of being clobbered.
- `report_error(msg)` — capture an unexpected error from anywhere. The only code
  that touches the `Sentry` global; swap the vendor here without touching call sites.

Configured at **compile time** via env vars (baked into the wasm bundle, like
landing's `NEXT_PUBLIC_*`). The DSN is public and safe to ship in the client.

```sh
CABINET_SENTRY_DSN=https://<key>@<org>.ingest.sentry.io/<project> \
CABINET_APP_ENV=production \
nix run .#cabinet
```

Unset or empty → monitoring is a no-op (local dev). Because `option_env!` doesn't
trigger a cargo rebuild on its own, a changed value needs `cargo clean -p cabinet`
to take effect. To bump the SDK or add an [SRI hash](https://docs.sentry.io/platforms/javascript/install/cdn/),
edit `SENTRY_SDK_URL` in `features/error_monitoring/boot.rs`.

## wasm-bindgen version pin

`Cargo.toml` pins `wasm-bindgen = "=0.2.118"` to match the `wasm-bindgen-cli` that
nixpkgs' `dioxus-cli` (`dx`) bundles — `dx` refuses to build on a version mismatch.
Bump it in lockstep whenever the pinned `dioxus-cli` changes its CLI version.
