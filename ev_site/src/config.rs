use serde::Deserialize;
#[cfg(not(target_arch = "wasm32"))]
use v_utils::macros as v_macros;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Default, v_macros::LiveSettings, v_macros::MyConfigPrimitives, v_macros::Settings)]
pub struct AppConfig {
	#[primitives(skip)]
	#[serde(default)]
	pub colorscheme: PreconfiguredColorscheme,
	#[serde(default)]
	pub port: u16 = 50736,
}

/// A color in the OKLCH color space (hue in degrees). `Display` emits the CSS `oklch()` form.
#[derive(Clone, Copy, Debug)]
pub struct Oklch {
	pub l: f64,
	pub c: f64,
	pub h: f64,
}
/// One concrete palette. The struct shape is global — every `public/colorschemes/*.nix` must define
/// exactly this field set; `build.rs` enforces that and emits one `COLORSCHEME_<NAME>` const per file.
#[derive(Clone, Copy, Debug)]
pub struct Colorscheme {
	pub bg_deep: Oklch,
	pub bg: Oklch,
	pub surface: Oklch,
	pub elevated: Oklch,
	pub border: Oklch,
	pub muted: Oklch,
	pub subtle: Oklch,
	pub text: Oklch,
	pub brand: Oklch,
	pub brand_fg: Oklch,
	pub brand_hi: Oklch,
	pub danger: Oklch,
	pub warning: Oklch,
	pub success: Oklch,
	pub info: Oklch,
}
/// User-facing selector for one of the colorschemes bundled at compile time from `public/colorschemes/`.
#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreconfiguredColorscheme {
	#[default]
	Dark,
	Light,
}
impl PreconfiguredColorscheme {
	pub fn colors(self) -> &'static Colorscheme {
		match self {
			Self::Dark => &COLORSCHEME_DARK,
			Self::Light => &COLORSCHEME_LIGHT,
		}
	}
}

impl std::fmt::Display for Oklch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "oklch({:.3} {:.3} {:.1})", self.l, self.c, self.h)
	}
}

include!(concat!(env!("OUT_DIR"), "/colorschemes.rs"));
