#!/usr/bin/env nix
---cargo
#! nix shell --impure --expr ``
#! nix let rust_flake = builtins.getFlake ''github:oxalica/rust-overlay'';
#! nix     nixpkgs_flake = builtins.getFlake ''nixpkgs'';
#! nix     pkgs = import nixpkgs_flake {
#! nix       system = builtins.currentSystem;
#! nix       overlays = [rust_flake.overlays.default];
#! nix     };
#! nix     toolchain = pkgs.rust-bin.nightly."2025-10-10".default.override {
#! nix       extensions = ["rust-src"];
#! nix     };
#! nix
#! nix in pkgs.symlinkJoin {
#! nix   name = "env";
#! nix   paths = [ toolchain pkgs.wtype ];
#! nix }
#! nix ``
#! nix --command sh -c ``cargo -Zscript -q "$0" "$@"``

[dependencies]
clap = { version = "4.5", features = ["derive"] }
---



//! Render a colorscheme draft as:
//!   1. horizontal swatches (top of HTML), one row per named color
//!   2. a hand-styled example hero page below, using those same colors
//!
//! Run from repo root:
//!   ./examples/colorscheme/explore.rs [--colorscheme PATH]
//!
//! The palette is loaded from a .nix file (flat attrset of `{ L; C; H; }`),
//! defaulting to `public/colorschemes/dark.nix`. Recipe behind the values
//! is in log/colorscheme.md.

use std::{
	fmt::Write as _,
	fs,
	path::{Path, PathBuf},
	process::Command,
	time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;

// ---------- OKLCH -> sRGB hex (D65) ----------------------------------------

fn oklch_to_linear_srgb(l: f64, c: f64, h_deg: f64) -> (f64, f64, f64) {
	let h = h_deg.to_radians();
	let a = c * h.cos();
	let b = c * h.sin();

	let l_ = l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
	let m_ = l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
	let s_ = l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;

	let (l3, m3, s3) = (l_.powi(3), m_.powi(3), s_.powi(3));

	let r = 4.076_741_662_1 * l3 - 3.307_711_591_3 * m3 + 0.230_969_929_2 * s3;
	let g = -1.268_438_004_6 * l3 + 2.609_757_401_1 * m3 - 0.341_319_396_5 * s3;
	let b_ = -0.004_196_086_3 * l3 - 0.703_418_614_7 * m3 + 1.707_614_701_0 * s3;
	(r, g, b_)
}

fn linear_to_srgb(c: f64) -> f64 {
	let c = c.clamp(0.0, 1.0);
	if c <= 0.003_130_8 { 12.92 * c } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}

fn oklch_hex(l: f64, c: f64, h: f64) -> String {
	let (r, g, b) = oklch_to_linear_srgb(l, c, h);
	let to_byte = |x: f64| (linear_to_srgb(x) * 255.0).round().clamp(0.0, 255.0) as u8;
	format!("#{:02X}{:02X}{:02X}", to_byte(r), to_byte(g), to_byte(b))
}

fn oklch_css(l: f64, c: f64, h: f64) -> String {
	format!("oklch({l:.3} {c:.3} {h:.1})")
}

// ---------- Palette loaded from a .nix file --------------------------------

struct Swatch {
	name: String,
	l: f64,
	c: f64,
	h: f64,
}

/// Parse the small fixed format used by `public/colorschemes/*.nix`:
///   `<name> = { L = <num>; C = <num>; H = <num>; };`
/// Lines that don't match (blank, comment, braces) are skipped.
fn load_palette(path: &Path) -> Vec<Swatch> {
	let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

	let mut out = Vec::new();
	for raw in text.lines() {
		let line = raw.split('#').next().expect("split always yields one item").trim();
		if line.is_empty() || !line.contains('=') || !line.contains('{') {
			continue;
		}

		let (name, rest) = line.split_once('=').expect("checked contains '='");
		let name = name.trim().to_string();

		// rest: "{ L = X; C = Y; H = Z; };"
		let inner = rest
			.trim()
			.strip_prefix('{')
			.unwrap_or_else(|| panic!("expected '{{' in: {line}"))
			.trim()
			.trim_end_matches(';')
			.trim()
			.strip_suffix('}')
			.unwrap_or_else(|| panic!("expected '}}' in: {line}"))
			.trim();

		let mut l = None;
		let mut c = None;
		let mut h = None;
		for assign in inner.split(';') {
			let assign = assign.trim();
			if assign.is_empty() {
				continue;
			}
			let (k, v) = assign.split_once('=').unwrap_or_else(|| panic!("expected 'k = v' in: {assign}"));
			let v: f64 = v.trim().parse().unwrap_or_else(|e| panic!("parse {v:?} as f64: {e}"));
			match k.trim() {
				"L" => l = Some(v),
				"C" => c = Some(v),
				"H" => h = Some(v),
				other => panic!("unknown key {other:?} in {line}"),
			}
		}

		out.push(Swatch {
			name,
			l: l.expect("L set on every line"),
			c: c.expect("C set on every line"),
			h: h.expect("H set on every line"),
		});
	}

	assert!(!out.is_empty(), "no swatches parsed from {}", path.display());
	out
}

// ---------- Swatches as plain HTML -----------------------------------------

fn build_swatch_html(palette: &[Swatch]) -> String {
	let mut rows = String::new();
	for sw in palette {
		let hex = oklch_hex(sw.l, sw.c, sw.h);
		let label = format!("{:<10}  L={:.3}  C={:.3}  H={:>5.1}", sw.name, sw.l, sw.c, sw.h);
		write!(
			rows,
			r##"<div class="swrow">
  <div class="swbar" style="background:{hex}"></div>
  <div class="swlabel">{label}  {hex}</div>
</div>
"##
		)
		.expect("write into String never fails");
	}

	format!(
		r##"<style>
  .swatches {{ background:#111; padding:20px; border-radius:8px; }}
  .swrow {{ display:flex; align-items:center; height:36px; margin:4px 0; }}
  .swbar {{ width:38%; height:100%; border-radius:4px; }}
  .swlabel {{
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    color:#e5e5e5; font-size:13px; margin-left:14px; white-space:pre;
  }}
</style>
<div class="swatches">
{rows}</div>"##
	)
}

// ---------- Hero example HTML ----------------------------------------------

fn build_hero_html(palette: &[Swatch]) -> String {
	let css_vars: String = palette
		.iter()
		.map(|s| format!("      --{}: {};", s.name, oklch_css(s.l, s.c, s.h)))
		.collect::<Vec<_>>()
		.join("\n");

	format!(
		r##"
    <style>
      .demo-root {{
{css_vars}
        --radius: 8px;
        /* Paper grain — inline SVG noise, layered above each surface's solid
           color. Alpha is tiny on purpose (reads as fiber, not texture). */
        --grain: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='220' height='220'><filter id='n'><feTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='2' stitchTiles='stitch'/><feColorMatrix values='0 0 0 0 0  0 0 0 0 0  0 0 0 0 0  0 0 0 0.05 0'/></filter><rect width='100%' height='100%' filter='url(%23n)'/></svg>");
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        color: var(--text);
        background: var(--grain), var(--bg);
        padding: 0;
        margin: 0 auto;
        max-width: 1100px;
        border: 1px solid var(--border);
        border-radius: 12px;
        overflow: hidden;
      }}
      .demo-root * {{ box-sizing: border-box; }}
      .nav {{
        display: flex; align-items: center; justify-content: space-between;
        padding: 16px 28px; background: var(--grain), var(--bg_deep);
        border-bottom: 1px solid var(--border);
      }}
      .nav .logo {{
        display: inline-flex; align-items: center; gap: 10px;
        font-weight: 700; letter-spacing: 0.04em; color: var(--text);
      }}
      .nav .logo .mark {{
        width: 28px; height: 28px; border-radius: 6px;
        background: var(--brand);
        display: inline-flex; align-items: center; justify-content: center;
        color: var(--text); font-size: 13px; font-weight: 800;
      }}
      .nav a {{ color: var(--subtle); margin-left: 22px; text-decoration: none; font-size: 14px; }}
      .nav a:hover {{ color: var(--brand_fg); }}

      .hero {{
        padding: 64px 48px 48px; background: var(--grain), var(--elevated);
        border-top: 4px solid var(--brand);
      }}
      .hero h1 {{
        margin: 0 0 16px; font-size: 44px; line-height: 1.1; color: var(--text);
        max-width: 640px;
      }}
      .hero p.lead {{
        margin: 0 0 32px; font-size: 18px; color: var(--muted);
        max-width: 560px;
      }}
      .btn {{
        display: inline-flex; align-items: center; gap: 8px;
        padding: 12px 22px; border-radius: var(--radius);
        font-size: 15px; font-weight: 600; text-decoration: none;
        border: 1px solid transparent; cursor: pointer;
      }}
      .btn-primary {{ background: var(--brand_fg); color: var(--bg_deep); }}
      .btn-primary:hover {{ background: var(--brand_hi); }}
      .btn-ghost {{
        background: transparent; color: var(--text);
        border-color: var(--border);
      }}
      .btn-ghost:hover {{ border-color: var(--brand_fg); color: var(--brand_fg); }}

      .body {{ padding: 48px; background: var(--grain), var(--bg); }}
      .body h2 {{ margin: 0 0 12px; font-size: 22px; color: var(--text); }}
      .body p {{ color: var(--subtle); line-height: 1.6; }}
      .body p .muted {{ color: var(--muted); }}

      .cards {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; margin-top: 28px; }}
      .card {{
        background: var(--grain), var(--surface); border: 1px solid var(--border);
        border-radius: var(--radius); padding: 20px;
      }}
      .card h3 {{ margin: 0 0 8px; font-size: 15px; color: var(--text); }}
      .card p  {{ margin: 0; font-size: 13px; color: var(--muted); }}
      .card .num {{ font-size: 28px; font-weight: 700; color: var(--brand_fg); }}
      .card.elevated {{ background: var(--grain), var(--elevated); }}

      .states {{
        display: flex; flex-wrap: wrap; gap: 10px; margin-top: 28px;
      }}
      .pill {{
        display: inline-flex; align-items: center; gap: 8px;
        padding: 6px 12px; border-radius: 999px; font-size: 13px;
        background: var(--surface); border: 1px solid var(--border);
        color: var(--text);
      }}
      .pill .dot {{ width: 8px; height: 8px; border-radius: 50%; }}
      .pill.success .dot {{ background: var(--success); }}
      .pill.success {{ color: var(--success); border-color: color-mix(in oklch, var(--success) 40%, var(--border)); }}
      .pill.warning .dot {{ background: var(--warning); }}
      .pill.warning {{ color: var(--warning); border-color: color-mix(in oklch, var(--warning) 40%, var(--border)); }}
      .pill.danger  .dot {{ background: var(--danger);  }}
      .pill.danger  {{ color: var(--danger); border-color: color-mix(in oklch, var(--danger) 40%, var(--border)); }}
      .pill.info    .dot {{ background: var(--info);    }}
      .pill.info    {{ color: var(--info); border-color: color-mix(in oklch, var(--info) 40%, var(--border)); }}

      .formrow {{ display: flex; gap: 12px; margin-top: 24px; align-items: center; }}
      .formrow input {{
        flex: 1; padding: 11px 14px; border-radius: var(--radius);
        background: var(--bg_deep); border: 1px solid var(--border);
        color: var(--text); font-size: 14px; outline: none;
      }}
      .formrow input::placeholder {{ color: var(--muted); }}
      .formrow input:focus {{ border-color: var(--brand_fg); box-shadow: 0 0 0 3px color-mix(in oklch, var(--brand_fg) 25%, transparent); }}

      .footer {{
        padding: 22px 48px; border-top: 1px solid var(--border);
        background: var(--bg_deep); color: var(--muted); font-size: 13px;
        display: flex; justify-content: space-between;
      }}
      .footer a {{ color: var(--subtle); text-decoration: none; }}
      .footer a:hover {{ color: var(--brand_fg); }}
      a.link {{ color: var(--brand_fg); }}
    </style>

    <div class="demo-root">
      <div class="nav">
        <span class="logo"><span class="mark">EV</span> EV INVESTMENT</span>
        <div>
          <a href="#">Strategy</a>
          <a href="#">Portfolio</a>
          <a href="#">Insights</a>
          <a href="#">Contact</a>
        </div>
      </div>

      <div class="hero">
        <h1>Capital, deployed with conviction.</h1>
        <p class="lead">
          Long-horizon investing in companies building the electric future.
          Disciplined research. Concentrated positions. No nonsense.
        </p>
        <a class="btn btn-primary" href="#">Read our thesis &rarr;</a>
        &nbsp;
        <a class="btn btn-ghost" href="#">Portfolio &rarr;</a>
      </div>

      <div class="body">
        <h2>Recent performance</h2>
        <p>
          Numbers below are illustrative. <span class="muted">Past returns are not a
          reliable indicator of future performance.</span> See our
          <a class="link" href="#">methodology &rarr;</a>.
        </p>

        <div class="cards">
          <div class="card">
            <h3>YTD return</h3>
            <div class="num">+18.4%</div>
            <p>vs benchmark +9.1%</p>
          </div>
          <div class="card elevated">
            <h3>AUM</h3>
            <div class="num">$240M</div>
            <p>across 11 positions</p>
          </div>
          <div class="card">
            <h3>Hold time</h3>
            <div class="num">4.2y</div>
            <p>median, conviction-weighted</p>
          </div>
        </div>

        <div class="states">
          <span class="pill success"><span class="dot"></span> Allocation healthy</span>
          <span class="pill info"><span class="dot"></span> Re-balance scheduled</span>
          <span class="pill warning"><span class="dot"></span> Position near cap</span>
          <span class="pill danger"><span class="dot"></span> Drawdown alert</span>
        </div>

        <div class="formrow">
          <input placeholder="your@email — quarterly memos" />
          <a class="btn btn-primary" href="#">Subscribe</a>
        </div>
      </div>

      <div class="footer">
        <span>&copy; EV Investment 2026</span>
        <span><a href="#">Privacy</a> &middot; <a href="#">Terms</a> &middot; <a href="#">Disclosures</a></span>
      </div>
    </div>
    "##
	)
}

// ---------- Compose final HTML ---------------------------------------------

#[derive(Parser)]
#[command(about = "Render a colorscheme as swatches + a hero example, open in browser")]
struct Args {
	/// Path to a colorscheme .nix file (flat attrset of `{ L; C; H; }` entries).
	#[arg(long, default_value = "public/colorschemes/dark.nix")]
	colorscheme: PathBuf,
}

fn main() {
	let args = Args::parse();

	let palette = load_palette(&args.colorscheme);
	let swatch_html = build_swatch_html(&palette);
	let hero_html = build_hero_html(&palette);

	let page = format!(
		r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>EV colorscheme exploration</title>
<style>
  body {{ margin: 0; background: #0a0a0a; color: #e5e5e5;
         font-family: -apple-system, BlinkMacSystemFont, sans-serif; }}
  h1.page {{ margin: 0; padding: 28px 48px 8px; font-size: 20px; font-weight: 600; }}
  .section-title {{ padding: 8px 48px 4px; color: #888; font-size: 13px;
                    letter-spacing: 0.08em; text-transform: uppercase; }}
  .wrap {{ padding: 12px 28px 48px; }}
</style>
</head>
<body>
  <h1 class="page">EV colorscheme — exploration ({source})</h1>
  <div class="section-title">Palette (OKLCH)</div>
  <div class="wrap">{swatch_html}</div>
  <div class="section-title">Hero example using the same colors</div>
  <div class="wrap">{hero_html}</div>
</body>
</html>
"#,
		source = args.colorscheme.display(),
	);

	let nanos = SystemTime::now().duration_since(UNIX_EPOCH).expect("monotonic since epoch").as_nanos();
	let dir = std::env::temp_dir().join(format!("colorscheme-{nanos}"));
	fs::create_dir_all(&dir).expect("create temp dir");
	let out: PathBuf = dir.join("explore.html");
	fs::write(&out, page).expect("write explore.html");
	println!("wrote {}", out.display());

	let uri = format!("file://{}", out.display());
	let status = Command::new("xdg-open").arg(&uri).status().expect("xdg-open must be on PATH to open browser");
	assert!(status.success(), "xdg-open exited with {status}");
}
