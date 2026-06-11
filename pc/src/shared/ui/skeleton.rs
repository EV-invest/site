use dioxus::prelude::*;

use crate::cn;

#[component]
pub fn Skeleton(#[props(default)] class: String) -> Element {
	let cls = cn!("animate-pulse rounded-md bg-muted", class);
	rsx! {
		div { class: cls }
	}
}
