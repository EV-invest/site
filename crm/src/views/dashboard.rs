use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
	rsx! {
		div {
			h1 {
				style: "font-size: 1.5rem; font-weight: 700; margin-bottom: 1.5rem;",
				"Dashboard"
			}
			p { style: "color: var(--text-muted);", "Portfolio overview — coming soon." }
		}
	}
}
