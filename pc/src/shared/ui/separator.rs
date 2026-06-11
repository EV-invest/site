use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Default)]
pub enum Orientation {
	#[default]
	Horizontal,
	Vertical,
}

#[component]
pub fn Separator(#[props(default)] orientation: Orientation, #[props(default)] class: String) -> Element {
	let cls = match orientation {
		Orientation::Horizontal => cn!("shrink-0 bg-border h-px w-full", class),
		Orientation::Vertical => cn!("shrink-0 bg-border w-px self-stretch", class),
	};
	rsx! {
		div { role: "separator", class: cls }
	}
}
