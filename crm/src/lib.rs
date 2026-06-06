mod views;

use dioxus::prelude::*;
use views::{Clients, Dashboard, Investments, NotFound};

const MAIN_CSS: Asset = asset!("/assets/main.css", CssAssetOptions::new());

#[component]
pub fn App() -> Element {
	rsx! {
		document::Stylesheet { href: MAIN_CSS }
		Router::<Route> {}
	}
}

#[derive(Clone, PartialEq, Routable, Debug)]
#[rustfmt::skip]
enum Route {
	#[layout(Layout)]
		#[route("/")]
		Dashboard {},
		#[route("/clients")]
		Clients {},
		#[route("/investments")]
		Investments {},
	#[end_layout]

	#[route("/:..route")]
	NotFound { route: Vec<String> },
}

#[component]
fn Layout() -> Element {
	rsx! {
		div {
			id: "main",

			nav {
				style: "width: var(--sidebar-width); \
				        background: var(--bg-surface); \
				        border-right: 1px solid var(--border); \
				        display: flex; flex-direction: column; \
				        padding: 1.25rem 0.75rem; \
				        gap: 0.25rem; flex-shrink: 0;",

				div {
					style: "padding: 0 0.75rem 1rem; \
					        font-weight: 700; font-size: 1rem; \
					        letter-spacing: 0.02em; color: var(--text); \
					        border-bottom: 1px solid var(--border); \
					        margin-bottom: 0.5rem;",
					"EV Fund CRM"
				}

				Link { class: "nav-link", to: Route::Dashboard {}, active_class: "active", "Dashboard" }
				Link { class: "nav-link", to: Route::Clients {}, active_class: "active", "Clients" }
				Link { class: "nav-link", to: Route::Investments {}, active_class: "active", "Investments" }
			}

			div {
				style: "flex: 1; overflow-y: auto; padding: 2rem;",
				Outlet::<Route> {}
			}
		}
	}
}
