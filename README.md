# ev_site
![Minimum Supported Rust Version](https://img.shields.io/badge/nightly-1.92+-ab6000.svg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ev_site.svg?color=fc8d62&logo=rust" height="20" style=flat-square>](https://crates.io/crates/ev_site)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs&style=flat-square" height="20">](https://docs.rs/ev_site)
![Lines Of Code](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/ev_site-loc.json)
<br>
[<img alt="ci errors" src="https://img.shields.io/github/actions/workflow/status/valeratrades/ev_site/errors.yml?branch=master&style=for-the-badge&style=flat-square&label=errors&labelColor=420d09" height="20">](https://github.com/valeratrades/ev_site/actions?query=branch%3Amaster) <!--NB: Won't find it if repo is private-->
[<img alt="ci warnings" src="https://img.shields.io/github/actions/workflow/status/valeratrades/ev_site/warnings.yml?branch=master&style=for-the-badge&style=flat-square&label=warnings&labelColor=d16002" height="20">](https://github.com/valeratrades/ev_site/actions?query=branch%3Amaster) <!--NB: Won't find it if repo is private-->

site of `EV Investment` fund

## Usage
The site lives in [`frontend/`](./docs/.readme_assets/frontend) — Next.js (App Router) + React + npm,
laid out with Feature-Sliced Design. It serves on `:3000`.

### Dev
One command brings it up without first entering the dev shell — it resolves the
repo root at runtime, `npm install`s on first run, then `npm run dev`:
```sh
nix run .#dev   # → http://localhost:3000
```
Or, from inside the dev shell (auto-activated by `.envrc` + direnv):
```sh
cd frontend && npm install && npm run dev
```

### Visual-regression tests
Per-section Playwright screenshot tests live in [`frontend/tests/`](./docs/.readme_assets/frontend/tests)
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

