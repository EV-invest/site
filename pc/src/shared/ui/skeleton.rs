use dioxus::prelude::*;

#[component]
pub fn Skeleton(#[props(default)] class: String) -> Element {
	let cls = format!("animate-pulse rounded-md bg-muted {class}");
	rsx! {
		div { class: cls }
	}
}
