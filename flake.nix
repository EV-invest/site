{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
    v_flakes.url = "github:valeratrades/v_flakes?ref=v1.6";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, pre-commit-hooks, v_flakes }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          allowUnfree = true;
        };
        #NB: can't load rust-bin from nightly.latest, as there are week guarantees of which components will be available on each day.
        rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" "rust-docs" "rustc-codegen-cranelift-preview" ];
          targets = [ "wasm32-unknown-unknown" ];
        });
        pre-commit-check = pre-commit-hooks.lib.${system}.run (v_flakes.files.preCommit { inherit pkgs; });
        manifest = (pkgs.lib.importTOML ./ev_site/Cargo.toml).package;
        pname = manifest.name;
        stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;

        github = v_flakes.github {
          inherit pkgs pname; #rs py;
          enable = true;
          lastSupportedVersion = "nightly-2026-05-12";
          jobs.default = true;
          lfs = false;
        };
        readme = v_flakes.readme-fw {
          inherit pkgs pname;
          defaults = true;
          lastSupportedVersion = "nightly-1.92";
          rootDir = ./.;
          badges = [ "msrv" "crates_io" "docs_rs" "loc" "ci" ];
        };
        combined = v_flakes.utils.combine [ github readme ];

        # ── dev orchestrator ────────────────────────────────────────────────
        # Self-contained wrapper so `nix run .#dev` starts the site without
        # first entering the devShell. The rust crate is being deprecated, so
        # "the site" is just ./frontend (vite + pnpm).
        #
        # IMPORTANT: resolve the repo at *runtime* via `git rev-parse`, not
        # `toString ./.`. Baking the latter locks the wrapper to the read-only
        # /nix/store snapshot, where pnpm can't write node_modules. Run from
        # anywhere inside the repo.
        #
        # pnpm is provisioned through corepack (matching the devShell), pinned
        # to the frontend's `packageManager` field. Shims live under a writable
        # .direnv/corepack so the read-only node install is never touched.
        runFrontend = pkgs.writeShellApplication {
          name = "run-frontend";
          runtimeInputs = with pkgs; [ nodejs corepack git ];
          text = ''
            repo="$(git rev-parse --show-toplevel)"
            export COREPACK_HOME="$repo/.direnv/corepack"
            mkdir -p "$COREPACK_HOME/bin"
            corepack enable --install-directory "$COREPACK_HOME/bin" pnpm
            export PATH="$COREPACK_HOME/bin:$PATH"

            cd "$repo/frontend"
            if [ ! -d node_modules ]; then
              pnpm install
            fi
            exec pnpm dev
          '';
        };
      in
      {
        apps.dev = {
          type = "app";
          program = "${runFrontend}/bin/run-frontend";
        };

        devShells.default =
          with pkgs;
          mkShell {
            shellHook =
              pre-commit-check.shellHook
              + combined.shellHook
              + ''
                cp -f ${(v_flakes.files.treefmt) { inherit pkgs; }} ./.treefmt.toml

                # Provision the exact pnpm pinned by the frontend's
                # `packageManager` field (pnpm@10.4.1) via corepack. nixpkgs'
                # pnpm tracks a single version and would not match the pin, so
                # we let corepack resolve it instead. Shims live under .direnv
                # (gitignored, writable) so the read-only /nix/store node
                # install is never touched. Run `pnpm install` in frontend/ to
                # populate node_modules.
                export COREPACK_HOME="$PWD/.direnv/corepack"
                mkdir -p "$COREPACK_HOME/bin"
                corepack enable --install-directory "$COREPACK_HOME/bin" pnpm
                export PATH="$COREPACK_HOME/bin:$PATH"
              '';

            packages = [
              corepack
              nodejs
              playwright-driver.browsers
            ] ++ pre-commit-check.enabledPackages ++ combined.enabledPackages;

            env.RUST_BACKTRACE = 1;
            env.RUST_LIB_BACKTRACE = 0;

            # Playwright: drive the nixpkgs-provided browsers instead of the
            # npm-downloaded ones (those are dynamically linked against libs
            # absent on NixOS). The npm @playwright/test version MUST match
            # playwright-driver's (1.59.1) or the browser revisions won't line up.
            env.PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers}";
            env.PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
            env.PLAYWRIGHT_HOST_PLATFORM_OVERRIDE = "nixos";
          };
      }
    );
}
