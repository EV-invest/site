The site lives in [`frontend/`](./frontend) — Vite + React + pnpm (pinned `pnpm@10.4.1`,
provisioned by corepack inside the dev shell). It serves on `:3000` with `--host`.

## Dev
One command brings it up without first entering the dev shell — it resolves the
repo root at runtime, provisions pnpm via corepack, `pnpm install`s on first run,
then `pnpm dev`:
```sh
nix run .#dev   # → http://localhost:3000
```
Or, from inside the dev shell (auto-activated by `.envrc` + direnv):
```sh
cd frontend && pnpm install && pnpm dev
```

## Visual-regression tests
Per-section Playwright screenshot tests live in [`frontend/tests/`](./frontend/tests)
— one baseline per addressable section (`#hero`, `#portfolio`, `#calculator`,
`#research`, `#team`, plus header/footer). Browsers come from nixpkgs (pinned to
the `@playwright/test` revision via the flake), so screenshots are reproducible.
```sh
cd frontend
pnpm test:visual          # compare against committed baselines
pnpm test:visual:update   # regenerate baselines after an intentional UI change
```

> The Rust crate (`ev_site`, `dx serve -p ev_site`) is being deprecated in favour
> of `frontend/`.
