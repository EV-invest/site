mod components;
mod pages;

use dioxus::prelude::*;
use pages::{Home, NotFound};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css", CssAssetOptions::new());

#[component]
pub fn App() -> Element {
	rsx! {
		document::Stylesheet { href: TAILWIND_CSS }
		Router::<Route> {}
	}
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
	#[route("/")]
	Home {},
	#[route("/:..route")]
	NotFound { route: Vec<String> },
}
