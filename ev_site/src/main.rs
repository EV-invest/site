#![feature(default_field_values)]
//! EV Investment site — Dioxus implementation.
//!
//! Layout & design mirror `examples/colorscheme/explore.py` (the hero page block).
//! Concrete user-facing strings live as `const`s at the top of this file. They will be
//! swapped for real data later.
//!
//! Styling convention: every element carries its own visual info — either via an inline
//! `style:` attribute, or via a sibling `<style>` block (for `:hover`, `:focus`,
//! `::placeholder`, descendant rules). The only globally-defined CSS is the colorscheme
//! variables and the document reset (see `build_root_css`). All lengths are in `rem`.

use dioxus::prelude::*;
use ev_site::config::{COLORSCHEME_MAIN, Colorscheme};

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

// Reused inline-style fragments. Kept here purely to avoid retyping the same long
// string across multiple call-sites; each is a single CSS declaration block. To move
// or delete a button/card, copy or remove its callsite — these consts can stay if
// unused (they're plain `&str`s) or be inlined back.
const STYLE_BTN_BASE: &str = "display: inline-flex; align-items: center; gap: 0.5rem; \
                              padding: 0.75rem 1.375rem; border-radius: var(--radius); \
                              font-size: 0.9375rem; font-weight: 600; text-decoration: none; \
                              border: 0.0625rem solid transparent; cursor: pointer;";
const STYLE_BTN_PRIMARY: &str = "background: var(--brand_fg); color: var(--bg_deep);";
const STYLE_BTN_GHOST: &str = "background: transparent; color: var(--text); \
                               border-color: var(--border);";

const STYLE_CARD: &str = "border: 0.0625rem solid var(--border); border-radius: var(--radius); \
                          padding: 1.25rem;";
const STYLE_CARD_H3: &str = "margin: 0 0 0.5rem; font-size: 0.9375rem; color: var(--text);";
const STYLE_CARD_NUM: &str = "font-size: 1.75rem; font-weight: 700; color: var(--brand_fg);";
const STYLE_CARD_P: &str = "margin: 0; font-size: 0.8125rem; color: var(--muted);";

const STYLE_PILL_DOT: &str = "width: 0.5rem; height: 0.5rem; border-radius: 50%;";

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

// ---------- Layout: nav + outlet + footer ----------------------------------

#[component]
fn Shell() -> Element {
	rsx! {
		div {
			style: "color: var(--text); background: var(--bg); margin: 0 auto; \
					max-width: 68.75rem; border: 0.0625rem solid var(--border); \
					border-radius: 0.75rem; overflow: hidden;",

			nav {
				style: "display: flex; align-items: center; justify-content: space-between; \
						padding: 1rem 1.75rem; background: var(--bg_deep); \
						border-bottom: 0.0625rem solid var(--border);",
				// Default link look inside the nav + hover. Logo overrides via inline style.
				style { {r#"
					nav a { color: var(--subtle); margin-left: 1.375rem; text-decoration: none; font-size: 0.875rem; }
					nav a:hover { color: var(--brand_fg); }
				"#} }

				Link {
					to: Route::Home {},
					style: "display: inline-flex; align-items: center; gap: 0.625rem; \
							font-weight: 700; letter-spacing: 0.04em; color: var(--text); \
							text-decoration: none; margin-left: 0;",
					span {
						style: "width: 1.75rem; height: 1.75rem; border-radius: 0.375rem; \
								background: var(--brand); display: inline-flex; \
								align-items: center; justify-content: center; \
								color: var(--text); font-size: 0.8125rem; font-weight: 800;",
						"{LOGO_MARK}"
					}
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

			footer {
				style: "padding: 1.375rem 3rem; border-top: 0.0625rem solid var(--border); \
						background: var(--bg_deep); color: var(--muted); font-size: 0.8125rem; \
						display: flex; justify-content: space-between;",
				style { {r#"
					footer a { color: var(--subtle); text-decoration: none; }
					footer a:hover { color: var(--brand_fg); }
				"#} }

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
		section {
			style: "padding: 4rem 3rem 3rem; background: var(--brand);",

			h1 {
				style: "margin: 0 0 1rem; font-size: 2.75rem; line-height: 1.1; \
						color: var(--text); max-width: 40rem;",
				"{HERO_TITLE}"
			}
			p {
				style: "margin: 0 0 2rem; font-size: 1.125rem; color: var(--brand_hi); \
						max-width: 35rem; opacity: 0.9;",
				"{HERO_LEAD}"
			}
			Link {
				to: Route::Thesis {},
				class: "btn-primary",
				style: "{STYLE_BTN_BASE} {STYLE_BTN_PRIMARY}",
				"Read our thesis \u{2192}"
			}
			" "
			Link {
				to: Route::Portfolio {},
				class: "btn-ghost",
				style: "{STYLE_BTN_BASE} {STYLE_BTN_GHOST}",
				"Portfolio \u{2192}"
			}
		}

		section {
			style: "padding: 3rem; background: var(--bg);",

			h2 {
				style: "margin: 0 0 0.75rem; font-size: 1.375rem; color: var(--text);",
				"{PERF_HEADING}"
			}
			p {
				style: "color: var(--subtle); line-height: 1.6;",
				"{PERF_INTRO} "
				span { style: "color: var(--muted);", "{PERF_DISCLAIMER}" }
				" See our "
				Link {
					to: Route::Methodology {},
					style: "color: var(--brand_fg);",
					"methodology \u{2192}"
				}
				"."
			}

			div {
				style: "display: grid; grid-template-columns: repeat(3, 1fr); \
						gap: 1.25rem; margin-top: 1.75rem;",

				div {
					style: "background: var(--surface); {STYLE_CARD}",
					h3 { style: "{STYLE_CARD_H3}", "{YTD_LABEL}" }
					div { style: "{STYLE_CARD_NUM}", "{YTD_VALUE}" }
					p { style: "{STYLE_CARD_P}", "{YTD_SUB}" }
				}
				div {
					style: "background: var(--elevated); {STYLE_CARD}",
					h3 { style: "{STYLE_CARD_H3}", "{AUM_LABEL}" }
					div { style: "{STYLE_CARD_NUM}", "{AUM_VALUE}" }
					p { style: "{STYLE_CARD_P}", "{AUM_SUB}" }
				}
				div {
					style: "background: var(--surface); {STYLE_CARD}",
					h3 { style: "{STYLE_CARD_H3}", "{HOLD_LABEL}" }
					div { style: "{STYLE_CARD_NUM}", "{HOLD_VALUE}" }
					p { style: "{STYLE_CARD_P}", "{HOLD_SUB}" }
				}
			}

			div {
				style: "display: flex; flex-wrap: wrap; gap: 0.625rem; margin-top: 1.75rem;",

				span {
					style: "display: inline-flex; align-items: center; gap: 0.5rem; \
							padding: 0.375rem 0.75rem; border-radius: 999rem; \
							font-size: 0.8125rem; background: var(--surface); \
							color: var(--success); \
							border: 0.0625rem solid color-mix(in oklch, var(--success) 40%, var(--border));",
					span { style: "{STYLE_PILL_DOT} background: var(--success);" }
					" {PILL_SUCCESS}"
				}
				span {
					style: "display: inline-flex; align-items: center; gap: 0.5rem; \
							padding: 0.375rem 0.75rem; border-radius: 999rem; \
							font-size: 0.8125rem; background: var(--surface); \
							color: var(--info); \
							border: 0.0625rem solid color-mix(in oklch, var(--info) 40%, var(--border));",
					span { style: "{STYLE_PILL_DOT} background: var(--info);" }
					" {PILL_INFO}"
				}
				span {
					style: "display: inline-flex; align-items: center; gap: 0.5rem; \
							padding: 0.375rem 0.75rem; border-radius: 999rem; \
							font-size: 0.8125rem; background: var(--surface); \
							color: var(--warning); \
							border: 0.0625rem solid color-mix(in oklch, var(--warning) 40%, var(--border));",
					span { style: "{STYLE_PILL_DOT} background: var(--warning);" }
					" {PILL_WARNING}"
				}
				span {
					style: "display: inline-flex; align-items: center; gap: 0.5rem; \
							padding: 0.375rem 0.75rem; border-radius: 999rem; \
							font-size: 0.8125rem; background: var(--surface); \
							color: var(--danger); \
							border: 0.0625rem solid color-mix(in oklch, var(--danger) 40%, var(--border));",
					span { style: "{STYLE_PILL_DOT} background: var(--danger);" }
					" {PILL_DANGER}"
				}
			}

			div {
				style: "display: flex; gap: 0.75rem; margin-top: 1.5rem; align-items: center;",
				// `:focus` and `::placeholder` can't live on the inline attribute, so they
				// sit alongside the input. Scoped via a one-off class so we don't catch
				// other inputs that might appear later.
				style { {r#"
					.subscribe-input::placeholder { color: var(--muted); }
					.subscribe-input:focus {
						border-color: var(--brand_fg);
						box-shadow: 0 0 0 0.1875rem color-mix(in oklch, var(--brand_fg) 25%, transparent);
					}
				"#} }

				input {
					class: "subscribe-input",
					style: "flex: 1; padding: 0.6875rem 0.875rem; border-radius: var(--radius); \
							background: var(--bg_deep); border: 0.0625rem solid var(--border); \
							color: var(--text); font-size: 0.875rem; outline: none;",
					placeholder: "{SUBSCRIBE_PLACEHOLDER}"
				}
				Link {
					to: Route::Subscribe {},
					class: "btn-primary",
					style: "{STYLE_BTN_BASE} {STYLE_BTN_PRIMARY}",
					"{SUBSCRIBE_CTA}"
				}
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
		section {
			style: "padding: 3rem; background: var(--bg);",
			h2 {
				style: "margin: 0 0 0.75rem; font-size: 1.375rem; color: var(--text);",
				"Page not found"
			}
			p {
				style: "color: var(--subtle); line-height: 1.6;",
				"No page at /{path}."
			}
		}
	}
}

fn placeholder_page(title: &'static str) -> Element {
	rsx! {
		section {
			style: "padding: 3rem; background: var(--bg);",
			h2 {
				style: "margin: 0 0 0.75rem; font-size: 1.375rem; color: var(--text);",
				"{title}"
			}
			p {
				style: "color: var(--subtle); line-height: 1.6;",
				"Coming soon."
			}
		}
	}
}

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
