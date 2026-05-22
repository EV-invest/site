//! Translation of `manus/client/src/pages/NotFound.tsx`.

use dioxus::prelude::*;
use lucide_dioxus::{CircleAlert, House};
use shadcn_ui::{Button, ButtonVariant, Card, CardContent};

use crate::Route;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
	let _ = route;
	let nav = navigator();
	rsx! {
		div { class: "min-h-screen w-full flex items-center justify-center bg-gradient-to-br from-slate-50 to-slate-100",
			Card { class: "w-full max-w-lg mx-4 shadow-lg border-0 bg-white/80 backdrop-blur-sm",
				CardContent { class: "pt-8 pb-8 text-center",
					div { class: "flex justify-center mb-6",
						div { class: "relative",
							div { class: "absolute inset-0 bg-red-100 rounded-full animate-pulse" }
							CircleAlert { class: "relative h-16 w-16 text-red-500" }
						}
					}

					h1 { class: "text-4xl font-bold text-slate-900 mb-2", "404" }
					h2 { class: "text-xl font-semibold text-slate-700 mb-4", "Page Not Found" }
					p { class: "text-slate-600 mb-8 leading-relaxed",
						"Sorry, the page you are looking for doesn't exist."
						br {}
						"It may have been moved or deleted."
					}

					div { class: "flex flex-col sm:flex-row gap-3 justify-center",
						Button {
							variant: ButtonVariant::Default,
							onclick: move |_| { nav.push(Route::Home {}); },
							House { class: "w-4 h-4 mr-2" }
							"Go Home"
						}
					}
				}
			}
		}
	}
}
