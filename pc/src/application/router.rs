use dioxus::prelude::*;

use crate::{
	application::layout::Layout,
	views::{Clients, Dashboard, Investments, NotFound},
};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css", CssAssetOptions::new());

#[component]
pub fn App() -> Element {
	rsx! {
		document::Stylesheet { href: TAILWIND_CSS }
		Router::<Route> {}
	}
}

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Dashboard {},
        #[route("/clients")]
        Clients {},
        #[route("/investments")]
        Investments {},
    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
