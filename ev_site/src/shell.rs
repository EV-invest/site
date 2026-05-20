use dioxus::prelude::*;

use crate::{
	Route,
	consts::{COMPANY, COPYRIGHT, LOGO_MARK},
};

// ---------- Layout: nav + outlet + footer ----------------------------------

#[component]
pub(crate) fn Shell() -> Element {
	rsx! {
		div {
			style: "color: var(--text); background: var(--bg); margin: 0 auto; \
					max-width: 68.75rem; border: 0.0625rem solid var(--border); \
					border-radius: 0.75rem; overflow: hidden;",

			nav {
				style: "display: flex; align-items: center; justify-content: space-between; \
						padding: 1rem 1.75rem; background: var(--bg_deep); \
						border-bottom: 0.0625rem solid var(--border);",
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
