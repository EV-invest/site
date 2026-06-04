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
        pname = "ev_site";

        rs = v_flakes.rs { inherit pkgs rust; };
        js = v_flakes.js {
          inherit pkgs;
          # Run `pnpm test:visual` (playwright) in the pre-commit hook.
          preCommit.visual = true;
        };
        github = v_flakes.github {
          inherit pkgs pname rs js;
          enable = true;
          lastSupportedVersion = "nightly-2026-05-12";
          jobs = {
            warnings.augment = [ "tokei" "code-duplication" ];
            other.augment = [ "loc-badge" ];
          };
          lfs = false;
        };
        readme = v_flakes.readme-fw {
          inherit pkgs pname;
          defaults = true;
          lastSupportedVersion = "nightly-1.92";
          rootDir = ./.;
          badges = [ "msrv" "crates_io" "docs_rs" "loc" "ci" ];
        };
        combined = v_flakes.utils.combine { inherit rust; modules = [ rs github readme ]; };

        # ── dev orchestrator ────────────────────────────────────────────────
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
            export COREPACK_ENABLE_DOWNLOAD_PROMPT=0
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

        # ── local Postgres ──────────────────────────────────────────────────
        # Project-local dev database. Cluster data + unix sockets live under
        # .pg/ at the repo root (gitignored), resolved at runtime like the
        # frontend wrapper. First run initdb's a trust-auth cluster and creates
        # the `ev_backend` database; subsequent runs just start the server.
        # Listens on 127.0.0.1:5432 — matches backend/.env.example.
        runPostgres = pkgs.writeShellApplication {
          name = "run-postgres";
          runtimeInputs = with pkgs; [ postgresql git coreutils gnugrep ];
          text = ''
            repo="$(git rev-parse --show-toplevel)"
            export PGDATA="$repo/.pg/data"
            sockets="$repo/.pg/sockets"
            port="''${PGPORT:-5432}"
            db="''${PGDATABASE:-ev_backend}"

            mkdir -p "$sockets"
            if [ ! -s "$PGDATA/PG_VERSION" ]; then
              echo "initialising postgres cluster in $PGDATA"
              initdb --username=postgres --auth=trust --pgdata="$PGDATA" >/dev/null
            fi

            # Create the app database once the server accepts connections; the
            # server itself stays in the foreground below.
            (
              until pg_isready --host="$sockets" --port="$port" --quiet; do sleep 0.2; done
              if ! psql --host="$sockets" --port="$port" --username=postgres --dbname=postgres \
                     --tuples-only --no-align \
                     --command "SELECT 1 FROM pg_database WHERE datname='$db'" | grep -q 1; then
                createdb --host="$sockets" --port="$port" --username=postgres "$db"
                echo "created database '$db'"
              fi
              echo "postgres ready on 127.0.0.1:$port (db '$db', user 'postgres', trust auth)"
            ) &

            exec postgres -D "$PGDATA" -k "$sockets" -h 127.0.0.1 -p "$port"
          '';
        };
      in
      {
        apps.dev = {
          type = "app";
          program = "${runFrontend}/bin/run-frontend";
        };

        # `nix run .#db` → boots local Postgres (see runPostgres above).
        apps.db = {
          type = "app";
          program = "${runPostgres}/bin/run-postgres";
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
                export COREPACK_ENABLE_DOWNLOAD_PROMPT=0
                export COREPACK_HOME="$PWD/.direnv/corepack"
                mkdir -p "$COREPACK_HOME/bin"
                corepack enable --install-directory "$COREPACK_HOME/bin" pnpm
                export PATH="$COREPACK_HOME/bin:$PATH"
              '';

            packages = [
              corepack
              nodejs
              openssl
              pkg-config
              rust
              mold
              postgresql
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
