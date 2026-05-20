use dioxus::prelude::*;

#[component]
pub(crate) fn PageNotFound(route: Vec<String>) -> Element {
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
