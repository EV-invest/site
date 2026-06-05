The site lives in [`frontend/`](./frontend) — Next.js (App Router) + React + npm,
laid out with Feature-Sliced Design. It serves on `:3000`.

## Dev
One command brings it up without first entering the dev shell — it resolves the
repo root at runtime, `npm install`s on first run, then `npm run dev`:
```sh
nix run .#dev   # → http://localhost:3000
```
Or, from inside the dev shell (auto-activated by `.envrc` + direnv):
```sh
cd frontend && npm install && npm run dev
```

## Visual-regression tests
Per-section Playwright screenshot tests live in [`frontend/tests/`](./frontend/tests)
— one baseline per addressable section (`#hero`, `#portfolio`, `#calculator`,
`#research`, `#team`, plus header/footer). Browsers come from nixpkgs (pinned to
the `@playwright/test` revision via the flake), so screenshots are reproducible.
```sh
cd frontend
npm run test:visual           # compare against committed baselines
npm run test:visual:update    # regenerate baselines after an intentional UI change
```

> The Rust crate (`ev_site`, `dx serve -p ev_site`) is being deprecated in favour
> of `frontend/`.
