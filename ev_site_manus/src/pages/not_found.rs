//! Translation of `manus/client/src/pages/NotFound.tsx`.

use dioxus::prelude::*;

use crate::Route;
use crate::icons::{AlertCircle, Home as HomeIcon};

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
	let _ = route;
	let nav = navigator();
	rsx! {
		div { class: "min-h-screen w-full flex items-center justify-center bg-gradient-to-br from-slate-50 to-slate-100",
			div { class: "w-full max-w-lg mx-4 shadow-lg border-0 bg-white/80 backdrop-blur-sm rounded-xl",
				div { class: "pt-8 pb-8 text-center px-6",
					div { class: "flex justify-center mb-6",
						div { class: "relative",
							div { class: "absolute inset-0 bg-red-100 rounded-full animate-pulse" }
							AlertCircle { class: "relative h-16 w-16 text-red-500" }
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
						button {
							class: "bg-blue-600 hover:bg-blue-700 text-white px-6 py-2.5 rounded-lg transition-all duration-200 shadow-md hover:shadow-lg inline-flex items-center justify-center",
							onclick: move |_| { nav.push(Route::Home {}); },
							HomeIcon { class: "w-4 h-4 mr-2" }
							"Go Home"
						}
					}
				}
			}
		}
	}
}
