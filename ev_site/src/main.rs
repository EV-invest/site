//! EV Investment site — Dioxus implementation.
//!
//! Layout & design mirror `examples/colorscheme/explore.py` (the hero page block).
//! Concrete user-facing strings live as `const`s at the top of this file. They will be
//! swapped for real data later.

pub mod config;

use config::{COLORSCHEME_MAIN, Colorscheme};
use dioxus::prelude::*;

// ---------- Placeholder copy (will be replaced by real data) ---------------

const COMPANY: &str = "EV INVESTMENT";
const LOGO_MARK: &str = "EV";

const HERO_TITLE: &str = "Capital, deployed with conviction.";
const HERO_LEAD: &str = "Long-horizon investing in companies building the electric future. \
                        Disciplined research. Concentrated positions. No nonsense.";

const PERF_HEADING: &str = "Recent performance";
const PERF_INTRO: &str = "Numbers below are illustrative.";
const PERF_DISCLAIMER: &str = "Past returns are not a reliable indicator of future performance.";

const YTD_LABEL: &str = "YTD return";
const YTD_VALUE: &str = "+18.4%";
const YTD_SUB: &str = "vs benchmark +9.1%";

const AUM_LABEL: &str = "AUM";
const AUM_VALUE: &str = "$240M";
const AUM_SUB: &str = "across 11 positions";

const HOLD_LABEL: &str = "Hold time";
const HOLD_VALUE: &str = "4.2y";
const HOLD_SUB: &str = "median, conviction-weighted";

const PILL_SUCCESS: &str = "Allocation healthy";
const PILL_INFO: &str = "Re-balance scheduled";
const PILL_WARNING: &str = "Position near cap";
const PILL_DANGER: &str = "Drawdown alert";

const SUBSCRIBE_PLACEHOLDER: &str = "your@email — quarterly memos";
const SUBSCRIBE_CTA: &str = "Subscribe";

const COPYRIGHT: &str = "\u{00A9} EV Investment 2026";

// ---------- Routes ---------------------------------------------------------

fn main() {
	dioxus::launch(App);
}
#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
	#[layout(Shell)]
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

#[component]
fn App() -> Element {
	rsx! {
		style { {build_css(&COLORSCHEME_MAIN)} }
		Router::<Route> {}
	}
}

// ---------- Layout: nav + outlet + footer ----------------------------------

#[component]
fn Shell() -> Element {
	rsx! {
		div { class: "demo-root",
			nav { class: "nav",
				Link { to: Route::Home {}, class: "logo",
					span { class: "mark", "{LOGO_MARK}" }
					" {COMPANY}"
				}
				div {
					Link { to: Route::Strategy {}, "Strategy" }
					Link { to: Route::Portfolio {}, "Portfolio" }
					Link { to: Route::Insights {}, "Insights" }
					Link { to: Route::Contact {}, "Contact" }
				}
			}

			Outlet::<Route> {}

			footer { class: "footer",
				span { "{COPYRIGHT}" }
				span {
					Link { to: Route::Privacy {}, "Privacy" }
					" \u{00B7} "
					Link { to: Route::Terms {}, "Terms" }
					" \u{00B7} "
					Link { to: Route::Disclosures {}, "Disclosures" }
				}
			}
		}
	}
}

// ---------- Home page ------------------------------------------------------

#[component]
fn Home() -> Element {
	rsx! {
		section { class: "hero",
			h1 { "{HERO_TITLE}" }
			p { class: "lead", "{HERO_LEAD}" }
			Link { to: Route::Thesis {}, class: "btn btn-primary", "Read our thesis \u{2192}" }
			" "
			Link { to: Route::Portfolio {}, class: "btn btn-ghost", "Portfolio \u{2192}" }
		}

		section { class: "body",
			h2 { "{PERF_HEADING}" }
			p {
				"{PERF_INTRO} "
				span { class: "muted", "{PERF_DISCLAIMER}" }
				" See our "
				Link { to: Route::Methodology {}, class: "link", "methodology \u{2192}" }
				"."
			}

			div { class: "cards",
				div { class: "card",
					h3 { "{YTD_LABEL}" }
					div { class: "num", "{YTD_VALUE}" }
					p { "{YTD_SUB}" }
				}
				div { class: "card elevated",
					h3 { "{AUM_LABEL}" }
					div { class: "num", "{AUM_VALUE}" }
					p { "{AUM_SUB}" }
				}
				div { class: "card",
					h3 { "{HOLD_LABEL}" }
					div { class: "num", "{HOLD_VALUE}" }
					p { "{HOLD_SUB}" }
				}
			}

			div { class: "states",
				span { class: "pill success", span { class: "dot" } " {PILL_SUCCESS}" }
				span { class: "pill info",    span { class: "dot" } " {PILL_INFO}" }
				span { class: "pill warning", span { class: "dot" } " {PILL_WARNING}" }
				span { class: "pill danger",  span { class: "dot" } " {PILL_DANGER}" }
			}

			div { class: "formrow",
				input { placeholder: "{SUBSCRIBE_PLACEHOLDER}" }
				Link { to: Route::Subscribe {}, class: "btn btn-primary", "{SUBSCRIBE_CTA}" }
			}
		}
	}
}

// ---------- Placeholder leaf pages -----------------------------------------

#[component]
fn Strategy() -> Element {
	placeholder_page("Strategy")
}

#[component]
fn Portfolio() -> Element {
	placeholder_page("Portfolio")
}

#[component]
fn Insights() -> Element {
	placeholder_page("Insights")
}

#[component]
fn Contact() -> Element {
	placeholder_page("Contact")
}

#[component]
fn Thesis() -> Element {
	placeholder_page("Thesis")
}

#[component]
fn Methodology() -> Element {
	placeholder_page("Methodology")
}

#[component]
fn Subscribe() -> Element {
	placeholder_page("Subscribe")
}

#[component]
fn Privacy() -> Element {
	placeholder_page("Privacy")
}

#[component]
fn Terms() -> Element {
	placeholder_page("Terms")
}

#[component]
fn Disclosures() -> Element {
	placeholder_page("Disclosures")
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
	let path = route.join("/");
	rsx! {
		section { class: "body",
			h2 { "Page not found" }
			p { "No page at /{path}." }
		}
	}
}

fn placeholder_page(title: &'static str) -> Element {
	rsx! {
		section { class: "body",
			h2 { "{title}" }
			p { "Coming soon." }
		}
	}
}

// ---------- CSS ------------------------------------------------------------
//
// Mirrors the inline stylesheet from `examples/colorscheme/explore.py::build_hero_html`.
// Color values come from the active `Colorscheme` via CSS custom properties so a
// future scheme switch only re-renders the `:root` block.

fn build_css(c: &Colorscheme) -> String {
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
  --radius: 8px;
}}

html, body {{
  margin: 0;
  background: var(--bg_deep);
  color: var(--text);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}}

.demo-root {{
  color: var(--text);
  background: var(--bg);
  margin: 0 auto;
  max-width: 1100px;
  border: 1px solid var(--border);
  border-radius: 12px;
  overflow: hidden;
}}
.demo-root * {{ box-sizing: border-box; }}

.nav {{
  display: flex; align-items: center; justify-content: space-between;
  padding: 16px 28px; background: var(--bg_deep);
  border-bottom: 1px solid var(--border);
}}
.nav .logo {{
  display: inline-flex; align-items: center; gap: 10px;
  font-weight: 700; letter-spacing: 0.04em; color: var(--text);
  text-decoration: none;
}}
.nav .logo .mark {{
  width: 28px; height: 28px; border-radius: 6px;
  background: var(--brand);
  display: inline-flex; align-items: center; justify-content: center;
  color: var(--text); font-size: 13px; font-weight: 800;
}}
.nav a {{ color: var(--subtle); margin-left: 22px; text-decoration: none; font-size: 14px; }}
.nav a:hover {{ color: var(--brand_fg); }}

.hero {{ padding: 64px 48px 48px; background: var(--brand); }}
.hero h1 {{
  margin: 0 0 16px; font-size: 44px; line-height: 1.1; color: var(--text);
  max-width: 640px;
}}
.hero p.lead {{
  margin: 0 0 32px; font-size: 18px; color: var(--brand_hi);
  max-width: 560px; opacity: 0.9;
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

.body {{ padding: 48px; background: var(--bg); }}
.body h2 {{ margin: 0 0 12px; font-size: 22px; color: var(--text); }}
.body p {{ color: var(--subtle); line-height: 1.6; }}
.body p .muted {{ color: var(--muted); }}

.cards {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; margin-top: 28px; }}
.card {{
  background: var(--surface); border: 1px solid var(--border);
  border-radius: var(--radius); padding: 20px;
}}
.card h3 {{ margin: 0 0 8px; font-size: 15px; color: var(--text); }}
.card p  {{ margin: 0; font-size: 13px; color: var(--muted); }}
.card .num {{ font-size: 28px; font-weight: 700; color: var(--brand_fg); }}
.card.elevated {{ background: var(--elevated); }}

.states {{ display: flex; flex-wrap: wrap; gap: 10px; margin-top: 28px; }}
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
