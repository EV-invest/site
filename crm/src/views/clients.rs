use dioxus::prelude::*;

#[component]
pub fn Clients() -> Element {
	rsx! {
		div {
			h1 {
				style: "font-size: 1.5rem; font-weight: 700; margin-bottom: 1.5rem;",
				"Clients"
			}
			p { style: "color: var(--text-muted);", "Client list — coming soon." }
		}
	}
}
