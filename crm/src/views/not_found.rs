use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
	let _ = route;
	let nav = navigator();
	rsx! {
		div {
			style: "min-height: 100vh; display: flex; align-items: center; \
			        justify-content: center; background: var(--bg);",
			div {
				style: "text-align: center; padding: 2rem;",
				h1 { style: "font-size: 4rem; font-weight: 800;", "404" }
				p {
					style: "font-size: 1.125rem; color: var(--text-muted); margin-bottom: 1.5rem;",
					"Page not found."
				}
				button {
					style: "padding: 0.625rem 1.25rem; background: var(--brand); color: #fff; \
					        border: none; border-radius: var(--radius); cursor: pointer; \
					        font-size: 0.9375rem; font-weight: 500;",
					onclick: move |_| { nav.push(Route::Dashboard {}); },
					"Go to Dashboard"
				}
			}
		}
	}
}
