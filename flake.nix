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
        # v_flakes.files.preCommit is a plain git-hooks.nix config — extend it
        # with project hooks by merging into `hooks` before handing it to run.
        pre-commit-check = pre-commit-hooks.lib.${system}.run (
          let base = v_flakes.files.preCommit { inherit pkgs; };
          in base // {
            hooks = base.hooks // {
              gen-api = {
                enable = true;
                # Regenerates landing/shared/api/generated from the committed
                # backend/openapi.json, then re-stages (same contract as the
                # treefmt hook above it in v_flakes).
                entry = "bash -c 'npm run gen:api && git add -u' --";
                pass_filenames = false;
                require_serial = true;
              };
            };
          }
        );
        pname = "ev_site";

        rs = v_flakes.rs { inherit pkgs rust; };
        js = v_flakes.js {
          inherit pkgs;
          preCommit.visual = false;
        };
        github = v_flakes.github {
          inherit pkgs pname rs js;
          enable = true;
          lastSupportedVersion = "nightly-2026-05-12";
          gitignore.extra = ''
            ## Local Postgres
            .pg/
            ## LLMs
            AGENTS.md
            CLAUDE.md
          '';
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

        # ── per-app dev wrappers ────────────────────────────────────────────
        # IMPORTANT: resolve the repo at *runtime* via `git rev-parse`, not
        # `toString ./.`. Baking the latter locks the wrapper to the read-only
        # /nix/store snapshot, where npm can't write node_modules. Run from
        # anywhere inside the repo. npm ships with the nixpkgs `nodejs`.

        # Linker shim shared by every wrapper that compiles Rust on macOS.
        # See the devShell shellHook for the full rationale — rust-lld embeds the
        # wrong libLLVM rpath, and only the FALLBACK var fixes it without forcing
        # Nix's clang onto rustc's older libLLVM during host proc-macro links.
        dyldFallback = ''export DYLD_FALLBACK_LIBRARY_PATH="${rust}/lib''${DYLD_FALLBACK_LIBRARY_PATH:+:$DYLD_FALLBACK_LIBRARY_PATH}"'';

        # landing (Next.js)
        runLanding = pkgs.writeShellApplication {
          name = "run-landing";
          runtimeInputs = with pkgs; [ nodejs git ];
          text = ''
            repo="$(git rev-parse --show-toplevel)"
            cd "$repo/landing"
            if [ ! -x node_modules/.bin/next ] \
               || [ package-lock.json -nt node_modules/.package-lock.json ]; then
              npm ci
            fi
            exec npm run dev
          '';
        };

        # ── TigerBeetle Rust client assets ──────────────────────────────────
        # Builds the native C client library + header so the official
        # tigerbeetle Rust crate can link against them. The output is the
        # src/clients/rust/ directory with compiled assets in place, ready
        # to be used as a Cargo path dependency.
        tigerbeetleClient = pkgs.stdenv.mkDerivation {
          name = "tigerbeetle-client";
          src = pkgs.fetchzip {
            url = "https://github.com/tigerbeetle/tigerbeetle/archive/refs/tags/0.17.6.tar.gz";
            hash = "sha256-b519nsDbas+XOw3ulAnzpk2KwtJkeOC3e13urM2tUSM=";
          };
          nativeBuildInputs = [ pkgs.zig ];
          buildPhase = ''
            zig build clients:rust -Drelease
          '';
          installPhase = ''
            mkdir -p $out
            cp -r src/clients/rust/* $out/
          '';
        };

        # backend (Axum). Migrations run automatically on startup, so a reachable
        # Postgres is the only prerequisite (`.#db`, or `.#dev` which boots one
        # first). Defaults mirror backend/.env.example; any value already in the
        # environment (or a sourced .env) wins via `:-`.
        runBackend = pkgs.writeShellApplication {
          name = "run-backend";
          runtimeInputs = with pkgs; [ rust pkg-config openssl git tigerbeetle zig ];
          text = ''
            ${dyldFallback}
            repo="$(git rev-parse --show-toplevel)"
            cd "$repo"

            # Symlink the TigerBeetle Rust client (with pre-built native assets)
            # so the path dependency in backend/Cargo.toml resolves.
            tb_client_dir="$repo/backend/.tb-client"
            if [ ! -L "$tb_client_dir" ] || [ "$(readlink "$tb_client_dir")" != "${tigerbeetleClient}" ]; then
              rm -f "$tb_client_dir"
              ln -s "${tigerbeetleClient}" "$tb_client_dir"
            fi

            export DATABASE_URL="''${DATABASE_URL:-postgres://postgres@localhost:5432/ev_backend}"
            export BIND_ADDR="''${BIND_ADDR:-0.0.0.0:8080}"
            export RUST_LOG="''${RUST_LOG:-info,backend=debug}"
            export TIGERBEETLE_ADDRESS="''${TIGERBEETLE_ADDRESS:-127.0.0.1:3001}"
            export TIGERBEETLE_CLUSTER_ID="''${TIGERBEETLE_CLUSTER_ID:-0}"
            exec cargo run -p backend
          '';
        };

        # pc (Dioxus / WASM). Build Tailwind once, keep it rebuilding in the
        # background (the `@source` scan in pc/input.css picks up class names from
        # RSX), then serve. dx defaults to :8080 like the backend, so pin pc to
        # :3001 to avoid a clash under `.#dev`.
        runPc = pkgs.writeShellApplication {
          name = "run-pc";
          runtimeInputs = with pkgs; [ rust dioxus-cli nodejs git ];
          text = ''
            ${dyldFallback}
            repo="$(git rev-parse --show-toplevel)"
            cd "$repo"
            npm run pc:css
            npm run pc:css:watch & css=$!
            trap 'kill "$css" 2>/dev/null || true' EXIT INT TERM
            # `--interactive false`: dx's default full-screen TUI assumes it owns
            # the terminal and gets corrupted (stuck "0%", N/A) when it shares
            # stdout with the css watcher or the other `.#dev` processes. Plain
            # streaming logs are the right fit for a backgrounded dev process.
            exec dx serve --package pc --port "''${PC_PORT:-3001}" --interactive false
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
            # Postgres refuses to start unless PGDATA is 0700/0750. A stray umask
            # or a dir created by `mkdir` rather than initdb leaves it 0755 — clamp it.
            chmod 0700 "$PGDATA"

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

        # ── local TigerBeetle ────────────────────────────────────────────────
        # Project-local TigerBeetle for the financial ledger. Data lives under
        # .tb/ at the repo root (gitignored), resolved at runtime. First run
        # formats a single-replica cluster; subsequent runs just start the server.
        # Listens on 127.0.0.1:3001 — matches backend/.env.example.
        runTigerbeetle = pkgs.writeShellApplication {
          name = "run-tigerbeetle";
          runtimeInputs = with pkgs; [ tigerbeetle git ];
          text = ''
            repo="$(git rev-parse --show-toplevel)"
            export TB_DATA="$repo/.tb/data"
            port="''${TBPORT:-3001}"
            cluster_id="''${TBCLUSTER:-0}"
            data_file="$TB_DATA/''${cluster_id}_0.tigerbeetle"

            mkdir -p "$TB_DATA"
            if [ ! -f "$data_file" ]; then
              echo "formatting TigerBeetle data file (cluster=''${cluster_id}, replica=0, replica-count=1)"
              tigerbeetle format --cluster="$cluster_id" --replica=0 --replica-count=1 "$data_file"
            fi

            echo "TigerBeetle ready on 127.0.0.1:$port (cluster ''${cluster_id})"
            exec tigerbeetle start --addresses="127.0.0.1:$port" "$data_file"
          '';
        };

        # ── full dev orchestrator ───────────────────────────────────────────
        # `nix run .#dev` → Postgres + TigerBeetle + backend + landing + pc,
        # all together. Postgres starts first, then TigerBeetle; backend only
        # launches once both accept connections. A single trap tears the whole
        # tree down on Ctrl-C / exit.
        runDev = pkgs.writeShellApplication {
          name = "run-dev";
          runtimeInputs = with pkgs; [ postgresql git coreutils ];
          text = ''
            pids=()
            cleanup() {
              echo; echo "shutting down dev stack…"
              [ ''${#pids[@]} -gt 0 ] && kill "''${pids[@]}" 2>/dev/null || true
              wait 2>/dev/null || true
            }
            trap cleanup EXIT INT TERM

            echo "▶ postgres"
            ${runPostgres}/bin/run-postgres & pids+=($!)

            echo "  waiting for postgres on 127.0.0.1:''${PGPORT:-5432}…"
            until pg_isready --host=127.0.0.1 --port="''${PGPORT:-5432}" --quiet; do sleep 0.3; done

            echo "▶ tigerbeetle"
            ${runTigerbeetle}/bin/run-tigerbeetle & pids+=($!)

            echo "▶ backend  (:8080)"
            ${runBackend}/bin/run-backend & pids+=($!)
            echo "▶ landing  (:3000)"
            ${runLanding}/bin/run-landing & pids+=($!)
            echo "▶ pc       (:3001)"
            ${runPc}/bin/run-pc & pids+=($!)

            wait
          '';
        };
      in
      {
        # `nix run .#dev`     → everything (postgres + tigerbeetle + backend + landing + pc)
        # `nix run .#landing` → Next.js dev server only
        # `nix run .#backend` → Axum API only (needs a DB: `.#db` or `.#dev`)
        # `nix run .#pc`      → Dioxus app + Tailwind watch only
        # `nix run .#db`      → local Postgres only
        # `nix run .#tb`      → local TigerBeetle only
        # (`.#prod` deliberately omitted — docker-vs-nix still undecided.)
        apps = {
          dev = { type = "app"; program = "${runDev}/bin/run-dev"; };
          landing = { type = "app"; program = "${runLanding}/bin/run-landing"; };
          backend = { type = "app"; program = "${runBackend}/bin/run-backend"; };
          pc = { type = "app"; program = "${runPc}/bin/run-pc"; };
          db = { type = "app"; program = "${runPostgres}/bin/run-postgres"; };
          tb = { type = "app"; program = "${runTigerbeetle}/bin/run-tigerbeetle"; };
        };

        devShells.default =
          with pkgs;
          mkShell {
            shellHook =
              pre-commit-check.shellHook
              + combined.shellHook
              + ''
                cp -f ${(v_flakes.files.treefmt) { inherit pkgs; }} ./.treefmt.toml

                # rust-lld (wasm32 linker) embeds the wrong rpath on macOS — it looks for
                # libLLVM.dylib in bin/../lib/ but Nix puts it one level up in lib/.
                # Use the FALLBACK var: plain DYLD_LIBRARY_PATH is searched before a binary's
                # own rpath and forces Nix's clang onto rustc's older libLLVM when linking
                # host proc-macros (missing LLVM-21 symbols → abort). The fallback only kicks
                # in when normal resolution fails — exactly rust-lld's case, never clang's.
                export DYLD_FALLBACK_LIBRARY_PATH="${rust}/lib''${DYLD_FALLBACK_LIBRARY_PATH:+:$DYLD_FALLBACK_LIBRARY_PATH}"
              '';

            packages = [
              nodejs
              openssl
              pkg-config
              rust
              mold
              postgresql
              tigerbeetle
              zig
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
