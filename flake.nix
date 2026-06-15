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
            ## Local TigerBeetle
            .tb/
            .tb-client
            ## LLMs
            AGENTS.md
            CLAUDE.md
            .claude/
            .pre-commit-config.yaml
          '';
          jobs = {
            warnings.augment = [ "tokei" "code-duplication" ];
            other.augment = [ "loc-badge" ];
          };
          lfs = false;
        };
        readme = v_flakes.readme-fw {
          inherit pkgs pname;
          # Repo is `EV-invest/site`, but the project (pname/gist) keeps `ev_site`.
          repo = "EV-invest/site";
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
            # Sentry (features/error-monitoring). One project DSN drives both the
            # browser (NEXT_PUBLIC_) and server/edge runtimes. Sourced from the
            # app-scoped LANDING_SENTRY_DSN — NOT the shared SENTRY_DSN — so an
            # ambient SENTRY_DSN can't funnel landing + backend into one project
            # under `.#dev`. Override LANDING_SENTRY_DSN per-env; set it empty to
            # disable. Source-map upload (SENTRY_AUTH_TOKEN) is CI-only, never set here.
            export NEXT_PUBLIC_SENTRY_DSN="''${LANDING_SENTRY_DSN:-https://702d594df36cfbd3c5a711613d3981e7@o4511508657012736.ingest.de.sentry.io/4511508677066832}"
            export SENTRY_DSN="''${LANDING_SENTRY_DSN:-https://702d594df36cfbd3c5a711613d3981e7@o4511508657012736.ingest.de.sentry.io/4511508677066832}"
            export NEXT_PUBLIC_APP_ENV="''${NEXT_PUBLIC_APP_ENV:-development}"
            export APP_ENV="''${APP_ENV:-development}"
            if [ ! -x node_modules/.bin/next ] \
               || [ package-lock.json -nt node_modules/.package-lock.json ]; then
              npm ci
            fi
            exec npm run dev
          '';
        };

        # ── TigerBeetle Rust client assets ──────────────────────────────────
        # Upstream's precompiled zig (fully static on linux, system-libs-only
        # on macOS) instead of nixpkgs zig: the nixpkgs build dynamically links
        # a separate ~500MB libLLVM, turning the one-time client build below
        # into a multi-GB substituter download. The official tarball is ~50MB.
        zigBin =
          let
            dist = {
              x86_64-linux = { suffix = "x86_64-linux"; sha256 = "24aeeec8af16c381934a6cd7d95c807a8cb2cf7df9fa40d359aa884195c4716c"; };
              aarch64-linux = { suffix = "aarch64-linux"; sha256 = "f7a654acc967864f7a050ddacfaa778c7504a0eca8d2b678839c21eea47c992b"; };
              x86_64-darwin = { suffix = "x86_64-macos"; sha256 = "b0f8bdfb9035783db58dd6c19d7dea89892acc3814421853e5752fe4573e5f43"; };
              aarch64-darwin = { suffix = "aarch64-macos"; sha256 = "39f3dc5e79c22088ce878edc821dedb4ca5a1cd9f5ef915e9b3cc3053e8faefa"; };
            }.${system};
          in
          pkgs.stdenvNoCC.mkDerivation {
            pname = "zig-bin";
            version = "0.14.1";
            src = pkgs.fetchurl {
              url = "https://ziglang.org/download/0.14.1/zig-${dist.suffix}-0.14.1.tar.xz";
              inherit (dist) sha256;
            };
            dontConfigure = true;
            dontBuild = true;
            # static binary — patchelf/strip would be a no-op at best
            dontFixup = true;
            installPhase = ''
              mkdir -p $out/bin
              cp zig $out/bin/
              # zig resolves its std lib relative to the (symlink-resolved)
              # binary path, checking ../lib among others
              cp -r lib $out/lib
            '';
          };

        # Official precompiled server binary. nixpkgs' tigerbeetle lags behind
        # (0.17.2) and a cluster evicts any client released after it
        # (client_release_too_high) — the server must be >= the 0.17.6 client
        # built below. The release binaries are static (zig-built), so they
        # run on NixOS unpatched.
        tigerbeetleBin =
          let
            dist = {
              x86_64-linux = { file = "tigerbeetle-x86_64-linux.zip"; hash = "sha256-butV+rwsBnpLCCOV9KNzvCNCC8QbG/AR7ZRnl+Uyl7Y="; };
              aarch64-linux = { file = "tigerbeetle-aarch64-linux.zip"; hash = "sha256-JmsczIvW67WTrK0iCEDHcu9lhMyK84ZvhIs+lgL2bAs="; };
              x86_64-darwin = { file = "tigerbeetle-universal-macos.zip"; hash = "sha256-83nhQqHYu6PPKu4rH6rjD/J3hJinhXQ6b7C4hZ9//v8="; };
              aarch64-darwin = { file = "tigerbeetle-universal-macos.zip"; hash = "sha256-83nhQqHYu6PPKu4rH6rjD/J3hJinhXQ6b7C4hZ9//v8="; };
            }.${system};
          in
          pkgs.stdenvNoCC.mkDerivation {
            pname = "tigerbeetle-bin";
            version = "0.17.6";
            src = pkgs.fetchurl {
              url = "https://github.com/tigerbeetle/tigerbeetle/releases/download/0.17.6/${dist.file}";
              inherit (dist) hash;
            };
            nativeBuildInputs = [ pkgs.unzip ];
            # the zip contains the bare binary at its root
            unpackPhase = "unzip $src";
            dontConfigure = true;
            dontBuild = true;
            dontFixup = true;
            installPhase = ''
              mkdir -p $out/bin
              install -m755 tigerbeetle $out/bin/
            '';
          };

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
          # TigerBeetle pins its zig version exactly (0.17.6 → zig 0.14.1);
          # newer zig is rejected by build.zig with @compileError.
          nativeBuildInputs = [ zigBin pkgs.git ];
          # build.zig runs `git tag --merged HEAD^` at configure time and
          # unconditionally consumes 4+ version-shaped tags from the result;
          # the release tarball has no .git, so fabricate a history with
          # enough tags. They only feed lazily-evaluated fetch_release steps
          # that clients:rust never depends on, so the values are arbitrary.
          postPatch = ''
            git init -q
            git -c user.name=nix -c user.email=nix@localhost add -A
            git -c user.name=nix -c user.email=nix@localhost commit -qm base
            for v in 0.17.1 0.17.2 0.17.3 0.17.4 0.17.5; do git tag "$v"; done
            git -c user.name=nix -c user.email=nix@localhost commit -qm head --allow-empty
          '';
          buildPhase = ''
            export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-cache"
            # -Dgit-commit: report the real 0.17.6 tag commit instead of the
            # fabricated repo's HEAD.
            # -Dconfig-release*: without them the client stamps itself as dev
            # release 65535.0.0 and any versioned cluster evicts it on connect
            # (client_release_too_high).
            zig build clients:rust -Drelease \
              -Dgit-commit=64899c7a41fd3d74c68da7bb2efcb7d208abd5f2 \
              -Dconfig-release=0.17.6 -Dconfig-release-client-min=0.17.6
          '';
          installPhase = ''
            mkdir -p $out
            cp -r src/clients/rust/* $out/
            # own workspace root: belt to the `workspace.exclude` suspenders in
            # the consuming repo — if the exclude ever stops matching, cargo
            # errors loudly (multiple workspace roots) instead of silently
            # adopting the read-only store package as a workspace member.
            printf '\n[workspace]\n' >> $out/Cargo.toml
          '';
        };

        # Symlink the TigerBeetle Rust client (with pre-built native assets) so
        # the path dependency in the workspace Cargo.toml resolves — needed both
        # by run-backend and by plain cargo / rust-analyzer in the dev shell.
        # Lives at the repo root, NOT under backend/: cargo's workspace exclude
        # can never match a path inside a member's directory (member matching is
        # prefix-based and overrides excludes), so a vendored package there gets
        # adopted as a workspace member and manifest-rewriting pre-commit hooks
        # (cargo autoinherit) EROFS on the read-only store copy.
        linkTbClient = ''
          tb_client_dir="$(git rev-parse --show-toplevel)/.tb-client"
          if [ ! -L "$tb_client_dir" ] || [ "$(readlink "$tb_client_dir")" != "${tigerbeetleClient}" ]; then
            rm -rf "$tb_client_dir"
            ln -s "${tigerbeetleClient}" "$tb_client_dir"
          fi
        '';

        # backend (Axum). Migrations run automatically on startup, so a reachable
        # Postgres is the only prerequisite (`.#db`, or `.#dev` which boots one
        # first). Defaults mirror backend/.env.example; any value already in the
        # environment (or a sourced .env) wins via `:-`.
        runBackend = pkgs.writeShellApplication {
          name = "run-backend";
          runtimeInputs = with pkgs; [ rust pkg-config openssl git ];
          text = ''
            ${dyldFallback}
            repo="$(git rev-parse --show-toplevel)"
            cd "$repo"

            ${linkTbClient}

            export DATABASE_URL="''${DATABASE_URL:-postgres://postgres@localhost:5432/ev_backend}"
            export BIND_ADDR="''${BIND_ADDR:-0.0.0.0:8080}"
            export RUST_LOG="''${RUST_LOG:-info,backend=debug}"
            export TIGERBEETLE_ADDRESS="''${TIGERBEETLE_ADDRESS:-127.0.0.1:3001}"
            export TIGERBEETLE_CLUSTER_ID="''${TIGERBEETLE_CLUSTER_ID:-0}"
            # Sentry (src/error_reporter.rs) — config.rs reads SENTRY_DSN + APP_ENV;
            # init is disabled when the DSN is empty. Sourced from app-scoped
            # BACKEND_SENTRY_DSN — NOT the shared SENTRY_DSN — so an ambient
            # SENTRY_DSN can't funnel backend + landing into one project under
            # `.#dev`. Override BACKEND_SENTRY_DSN per-env; set it empty to disable.
            export SENTRY_DSN="''${BACKEND_SENTRY_DSN:-https://408591d5c466dc8ab815aaa3eeb1ee9f@o4511508657012736.ingest.de.sentry.io/4511508712259664}"
            export APP_ENV="''${APP_ENV:-development}"
            exec cargo run -p backend
          '';
        };

        # cabinet (Dioxus / WASM). Build Tailwind once, keep it rebuilding in the
        # background (the `@source` scan in cabinet/input.css picks up class names from
        # RSX), then serve. dx defaults to :8080 like the backend, so pin cabinet to
        # :3001 to avoid a clash under `.#dev`.
        runCabinet = pkgs.writeShellApplication {
          name = "run-cabinet";
          runtimeInputs = with pkgs; [ rust dioxus-cli nodejs git ];
          text = ''
            ${dyldFallback}
            repo="$(git rev-parse --show-toplevel)"
            cd "$repo"
            npm run cabinet:css
            npm run cabinet:css:watch & css=$!
            trap 'kill "$css" 2>/dev/null || true' EXIT INT TERM
            # Sentry (features/error_monitoring) — public browser DSN, baked into
            # the wasm bundle at compile time via option_env!. Override per-env by
            # exporting CABINET_SENTRY_DSN / CABINET_APP_ENV (export empty to
            # disable monitoring). option_env! doesn't trigger a cargo rebuild on
            # its own, so a changed value needs `cargo clean -p cabinet` to apply.
            export CABINET_SENTRY_DSN="''${CABINET_SENTRY_DSN:-https://83d2b337105b3e4651e9fe3fd41c844b@o4511508657012736.ingest.de.sentry.io/4511569023139920}"
            export CABINET_APP_ENV="''${CABINET_APP_ENV:-development}"
            # `--interactive false`: dx's default full-screen TUI assumes it owns
            # the terminal and gets corrupted (stuck "0%", N/A) when it shares
            # stdout with the css watcher or the other `.#dev` processes. Plain
            # streaming logs are the right fit for a backgrounded dev process.
            exec dx serve --package cabinet --port "''${CABINET_PORT:-3001}" --interactive false
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
          runtimeInputs = [ tigerbeetleBin pkgs.git ];
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
        # `nix run .#dev` → Postgres + TigerBeetle + backend + landing + cabinet,
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
            echo "▶ cabinet       (:3001)"
            ${runCabinet}/bin/run-cabinet & pids+=($!)

            wait
          '';
        };
      in
      {
        # `nix run .#dev`     → everything (postgres + tigerbeetle + backend + landing + cabinet)
        # `nix run .#landing` → Next.js dev server only
        # `nix run .#backend` → Axum API only (needs a DB: `.#db` or `.#dev`)
        # `nix run .#cabinet`      → Dioxus app + Tailwind watch only
        # `nix run .#db`      → local Postgres only
        # `nix run .#tb`      → local TigerBeetle only
        # (`.#prod` deliberately omitted — docker-vs-nix still undecided.)
        apps = {
          dev = { type = "app"; program = "${runDev}/bin/run-dev"; };
          landing = { type = "app"; program = "${runLanding}/bin/run-landing"; };
          backend = { type = "app"; program = "${runBackend}/bin/run-backend"; };
          cabinet = { type = "app"; program = "${runCabinet}/bin/run-cabinet"; };
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

                ${linkTbClient}
              '';

            packages = [
              nodejs
              openssl
              pkg-config
              rust
              mold
              postgresql
              tigerbeetleBin
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
