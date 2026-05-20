use dioxus::prelude::*;

use crate::{Route, consts::*};

// ---------- Home page ------------------------------------------------------

#[component]
pub(crate) fn Home() -> Element {
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
