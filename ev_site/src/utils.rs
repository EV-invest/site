use dioxus::prelude::*;

pub(crate) fn placeholder_page(title: &'static str) -> Element {
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
