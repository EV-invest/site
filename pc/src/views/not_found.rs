use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
	let _ = route;
	rsx! { div { "404" } }
}
