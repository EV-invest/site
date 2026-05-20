#![feature(default_field_values)]

pub mod config;
mod consts;
mod routes;
mod shell;
mod utils;

use config::{COLORSCHEME_MAIN, Colorscheme};
use dioxus::prelude::*;
use routes::{Contact, Disclosures, Home, Insights, Methodology, PageNotFound, Portfolio, Privacy, Strategy, Subscribe, Terms, Thesis};

// ---------- Routes ---------------------------------------------------------

#[component]
pub fn App() -> Element {
	rsx! {
		style { {build_root_css(&COLORSCHEME_MAIN)} }
		// Shared button hover behaviour — kept at app level because `.btn-*` classes
		// recur across many pages. Base look stays inline on each button.
		style { {r#"
			.btn-primary:hover { background: var(--brand_hi); }
			.btn-ghost:hover { border-color: var(--brand_fg); color: var(--brand_fg); }
		"#} }
		Router::<Route> {}
	}
}
#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
	#[layout(shell::Shell)]
		#[route("/")]
		Home {},

		#[route("/strategy")]
		Strategy {},

		#[route("/portfolio")]
		Portfolio {},

		#[route("/insights")]
		Insights {},

		#[route("/contact")]
		Contact {},

		#[route("/thesis")]
		Thesis {},

		#[route("/methodology")]
		Methodology {},

		#[route("/subscribe")]
		Subscribe {},

		#[route("/privacy")]
		Privacy {},

		#[route("/terms")]
		Terms {},

		#[route("/disclosures")]
		Disclosures {},

		#[route("/:..route")]
		PageNotFound { route: Vec<String> },
}

// ---------- Entry ----------------------------------------------------------

// ---------- Global CSS -----------------------------------------------------
//
// Only what truly has to be global lives here: the colorscheme variables (so
// switching schemes re-renders one place) and the document-wide reset. All
// element-specific styling lives co-located with its rsx above.

fn build_root_css(c: &Colorscheme) -> String {
	format!(
		r#"
:root {{
  --bg_deep: {bg_deep};
  --bg: {bg};
  --surface: {surface};
  --elevated: {elevated};
  --border: {border};
  --muted: {muted};
  --subtle: {subtle};
  --text: {text};
  --brand: {brand};
  --brand_fg: {brand_fg};
  --brand_hi: {brand_hi};
  --danger: {danger};
  --warning: {warning};
  --success: {success};
  --info: {info};
  --radius: 0.5rem;
}}

html, body {{
  margin: 0;
  background: var(--bg_deep);
  color: var(--text);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}}

*, *::before, *::after {{ box-sizing: border-box; }}
"#,
		bg_deep = c.bg_deep,
		bg = c.bg,
		surface = c.surface,
		elevated = c.elevated,
		border = c.border,
		muted = c.muted,
		subtle = c.subtle,
		text = c.text,
		brand = c.brand,
		brand_fg = c.brand_fg,
		brand_hi = c.brand_hi,
		danger = c.danger,
		warning = c.warning,
		success = c.success,
		info = c.info,
	)
}
