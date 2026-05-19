use std::{env, fs, path::PathBuf};

fn main() {
	git_version();
	log_directives();
	colorschemes();
}

fn git_version() {
	println!("cargo:rerun-if-changed=../.git/HEAD");
	println!("cargo:rerun-if-changed=../.git/refs/");
	// Embed git commit hash (fallback to "unknown" if git unavailable, e.g., in Nix sandbox)
	let git_hash = std::process::Command::new("git")
		.args(["rev-parse", "--short", "HEAD"])
		.output()
		.ok()
		.and_then(|o| if o.status.success() { Some(o.stdout) } else { None })
		.and_then(|stdout| String::from_utf8(stdout).ok())
		.map(|s| s.trim().to_string())
		.unwrap_or_else(|| "unknown".to_string());
	println!("cargo:rustc-env=GIT_HASH={git_hash}");
}

fn log_directives() {
	// Embed log directives if ../.cargo/log_directives exists
	println!("cargo:rerun-if-changed=../.cargo/log_directives");
	if let Ok(directives) = fs::read_to_string("../.cargo/log_directives") {
		let directives = directives.trim();
		if !directives.is_empty() {
			println!("cargo:rustc-env=LOG_DIRECTIVES={directives}");
		}
	}
}

/// Lift every `../public/colorschemes/*.nix` into a Rust `const COLORSCHEME_<NAME>: Colorscheme`,
/// emitted into `$OUT_DIR/colorschemes.rs` and `include!`d from `src/config.rs`.
///
/// The colorscheme `.nix` files are restricted to a flat attrset of OKLCH triples
/// (`name = { L = ..; C = ..; H = ..; };`). Richer schemas would warrant invoking `nix eval`,
/// which we deliberately avoid here to keep the build hermetic.
fn colorschemes() {
	let dir = PathBuf::from("../public/colorschemes");
	println!("cargo:rerun-if-changed={}", dir.display());

	let mut schemes: Vec<(String, Vec<(String, Oklch)>)> = Vec::new();
	let entries = fs::read_dir(&dir).unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()));
	for entry in entries {
		let path = entry.expect("dir entry").path();
		if path.extension().and_then(|s| s.to_str()) != Some("nix") {
			continue;
		}
		println!("cargo:rerun-if-changed={}", path.display());
		let name = path.file_stem().expect("file stem").to_str().expect("utf8 stem").to_owned();
		let content = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
		schemes.push((name, parse_oklch_attrset(&content, &path)));
	}
	schemes.sort_by(|a, b| a.0.cmp(&b.0));
	assert!(!schemes.is_empty(), "no colorschemes found under {}", dir.display());

	// All schemes must agree on the field set (the `Colorscheme` struct shape is global).
	let reference: Vec<&str> = schemes[0].1.iter().map(|(k, _)| k.as_str()).collect();
	for (name, pairs) in &schemes[1..] {
		let keys: Vec<&str> = pairs.iter().map(|(k, _)| k.as_str()).collect();
		assert_eq!(keys, reference, "colorscheme `{name}` field set disagrees with `{}`", schemes[0].0);
	}

	let mut out = String::new();
	for (name, pairs) in &schemes {
		out.push_str(&format!("pub const COLORSCHEME_{}: Colorscheme = Colorscheme {{\n", name.to_uppercase()));
		for (k, v) in pairs {
			out.push_str(&format!("\t{k}: Oklch {{ l: {:?}, c: {:?}, h: {:?} }},\n", v.l, v.c, v.h));
		}
		out.push_str("};\n");
	}

	let out_path = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("colorschemes.rs");
	fs::write(&out_path, out).expect("write colorschemes.rs");
}

#[derive(Clone, Copy)]
struct Oklch {
	l: f64,
	c: f64,
	h: f64,
}

/// Parse `{ name = { L = N; C = N; H = N; }; ... }`. Comments (`#`) and whitespace are ignored.
fn parse_oklch_attrset(content: &str, path: &std::path::Path) -> Vec<(String, Oklch)> {
	// Strip line comments, then flatten to a single whitespace-normalized string.
	let stripped: String = content.lines().map(|l| l.split('#').next().unwrap_or("")).collect::<Vec<_>>().join(" ");
	let mut s = stripped.trim();
	let bail = |msg: &str, rest: &str| -> ! { panic!("{}: parse error: {msg}; at: {:?}", path.display(), &rest.chars().take(60).collect::<String>()) };

	s = s.strip_prefix('{').unwrap_or_else(|| bail("expected outer `{`", s)).trim_start();
	let mut out = Vec::new();
	loop {
		s = s.trim_start();
		if let Some(rest) = s.strip_prefix('}') {
			s = rest.trim_start();
			assert!(s.is_empty(), "{}: trailing content after closing `}}`: {s:?}", path.display());
			return out;
		}

		let eq = s.find('=').unwrap_or_else(|| bail("expected `=` after field name", s));
		let name = s[..eq].trim().to_owned();
		s = s[eq + 1..].trim_start();
		s = s.strip_prefix('{').unwrap_or_else(|| bail("expected `{` opening OKLCH triple", s)).trim_start();

		let mut l = None;
		let mut c = None;
		let mut h = None;
		for _ in 0..3 {
			let eq2 = s.find('=').unwrap_or_else(|| bail("expected `=` inside triple", s));
			let key = s[..eq2].trim();
			s = s[eq2 + 1..].trim_start();
			let semi = s.find(';').unwrap_or_else(|| bail("expected `;` after triple value", s));
			let num: f64 = s[..semi].trim().parse().unwrap_or_else(|e| bail(&format!("invalid number: {e}"), &s[..semi]));
			s = s[semi + 1..].trim_start();
			match key {
				"L" => l = Some(num),
				"C" => c = Some(num),
				"H" => h = Some(num),
				_ => bail(&format!("unexpected key `{key}` in OKLCH triple (want L/C/H)"), s),
			}
		}
		s = s.strip_prefix('}').unwrap_or_else(|| bail("expected `}` closing triple", s)).trim_start();
		s = s.strip_prefix(';').unwrap_or_else(|| bail("expected `;` after triple", s)).trim_start();

		out.push((
			name,
			Oklch {
				l: l.unwrap_or_else(|| panic!("{}: missing L", path.display())),
				c: c.unwrap_or_else(|| panic!("{}: missing C", path.display())),
				h: h.unwrap_or_else(|| panic!("{}: missing H", path.display())),
			},
		));
	}
}
