use dioxus::prelude::*;

#[component]
pub fn Investments() -> Element {
	rsx! {
		div {
			h1 {
				style: "font-size: 1.5rem; font-weight: 700; margin-bottom: 1.5rem;",
				"Investments"
			}
			p { style: "color: var(--text-muted);", "Investment records — coming soon." }
		}
	}
}
