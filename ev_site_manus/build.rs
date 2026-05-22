//! Compile `tailwind.css` (Tailwind v4 sources) into `assets/tailwind.css` so
//! the `asset!("/assets/tailwind.css", …)` invocation in `lib.rs` resolves at
//! macro time. Reruns whenever the entry file, the bespoke component sheet,
//! or any `.rs` under `src/` changes (Tailwind's `@source` directive scans
//! Rust files for class strings).
//!
//! shadcn_ui ships as a git dependency, so its sources live under the cargo
//! git-checkout cache at a path we can't hard-code. We resolve that path via
//! `cargo metadata` and append it as an additional `@source` line so Tailwind
//! emits classes for every shadcn variant we actually instantiate.

use std::{path::PathBuf, process::Command};

fn main() {
	println!("cargo:rerun-if-changed=tailwind.css");
	println!("cargo:rerun-if-changed=styles.css");
	println!("cargo:rerun-if-changed=src");

	let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("set by cargo"));
	let entry_src = manifest_dir.join("tailwind.css");
	let assets_dir = manifest_dir.join("assets");
	let output_css = assets_dir.join("tailwind.css");

	std::fs::create_dir_all(&assets_dir).expect("create assets dir");

	// Locate the shadcn_ui (`ui`) source directory in the cargo git cache.
	let shadcn_ui_src = shadcn_ui_src_dir().expect("resolve shadcn_ui source dir");
	println!("cargo:rerun-if-changed={}", shadcn_ui_src.display());

	// Write a generated entry that pins the shadcn_ui @source absolutely.
	// We place it next to the original so relative `@import "./styles.css"`
	// keeps resolving; if dropped into `assets/` it would chase a non-existent
	// sibling.
	let generated_entry = manifest_dir.join(".tailwind.entry.css");
	let original = std::fs::read_to_string(&entry_src).expect("read tailwind.css");
	let augmented = format!("@source \"{}\";\n{}", shadcn_ui_src.display(), original);
	std::fs::write(&generated_entry, augmented).expect("write generated entry");

	let status = Command::new("tailwindcss")
		.arg("-i")
		.arg(&generated_entry)
		.arg("-o")
		.arg(&output_css)
		.status()
		.expect("`tailwindcss` not in PATH — enter the nix devshell (or install Tailwind v4)");

	assert!(status.success(), "tailwindcss build failed");
}

/// Resolve the on-disk source directory for the `ui` crate (shadcn_ui).
/// Cargo's git-dep checkouts live at a content-addressed path that we can't
/// hard-code; we ask cargo where it is via `cargo metadata`. We match by
/// `(name, source-prefix)` — the workspace embeds dependency declarations
/// with a bare `"name":"ui"` field, so a substring scan would resolve to the
/// wrong record. Proper JSON parsing avoids that.
fn shadcn_ui_src_dir() -> Option<PathBuf> {
	let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
		.args(["metadata", "--format-version=1"])
		.output()
		.ok()?;
	assert!(output.status.success(), "cargo metadata failed");
	let meta: serde_json::Value = serde_json::from_slice(&output.stdout).expect("cargo metadata json");
	let pkg = meta
		.get("packages")?
		.as_array()?
		.iter()
		.find(|p| p.get("name").and_then(|n| n.as_str()) == Some("ui") && p.get("source").and_then(|s| s.as_str()).is_some_and(|s| s.contains("shadcn-dioxus")))?;
	let manifest_path = PathBuf::from(pkg.get("manifest_path")?.as_str()?);
	Some(manifest_path.parent()?.join("src"))
}
