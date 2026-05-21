#![feature(default_field_values)]
mod components;
mod icons;
mod pages;
mod styles;

use dioxus::prelude::*;
use pages::{Home, NotFound};

#[component]
pub fn App() -> Element {
	rsx! {
		style { {styles::GLOBAL_CSS} }
		Router::<Route> {}
	}
}
#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
	#[route("/")]
	Home {},

	#[route("/:..route")]
	NotFound { route: Vec<String> },
}
