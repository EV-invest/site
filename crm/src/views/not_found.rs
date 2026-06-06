use dioxus::prelude::*;

use crate::application::router::Route;
use crate::shared::ui::{Button, ButtonVariant};

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let _ = route;
    let nav = navigator();
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-background",
            div { class: "text-center px-8 space-y-4",
                p { class: "text-8xl font-bold text-muted-foreground/30", "404" }
                h1 { class: "text-2xl font-semibold text-foreground", "Page not found" }
                p { class: "text-muted-foreground text-sm", "The page you're looking for doesn't exist." }
                Button {
                    variant: ButtonVariant::Default,
                    onclick: move |_| { nav.push(Route::Dashboard {}); },
                    "Go to Dashboard"
                }
            }
        }
    }
}
