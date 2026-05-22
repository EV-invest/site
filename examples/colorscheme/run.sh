#!/usr/bin/env bash
# Standalone wrapper for explore.rs: brings in a pinned nightly rust toolchain
# via nix (no dependency on any out-of-tree wrapper script) and runs the
# embedded cargo script.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

NIX_EXPR='let
  rust_flake = builtins.getFlake "github:oxalica/rust-overlay/7ed7e8c74be95906275805db68201e74e9904f07";
  nixpkgs_flake = builtins.getFlake "github:NixOS/nixpkgs/f61125a668a320878494449750330ca58b78c557";
  pkgs = import nixpkgs_flake {
    system = builtins.currentSystem;
    overlays = [ rust_flake.overlays.default ];
  };
in pkgs.rust-bin.nightly."2025-10-10".default.override { extensions = ["rust-src"]; }'

RUST_PATH=$(nix build --no-link --print-out-paths --impure --expr "$NIX_EXPR")

exec env PATH="$RUST_PATH/bin:$PATH" RUSTC_WRAPPER="" \
    "$RUST_PATH/bin/cargo" -Zscript -q "$SCRIPT_DIR/explore.rs" "$@"
