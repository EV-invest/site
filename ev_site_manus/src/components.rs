//! Shared building blocks for the manus port.
//!
//! Currently a single component: [`Button`], a thin port of shadcn-ui's
//! `Button` (manus/client/src/components/ui/button.tsx) restricted to the
//! `default` variant and `default` size — the only configuration the ported
//! pages actually use. It emits the same baseline utility classes that
//! `buttonVariants` injects, so the cascade interaction with `.btn-primary` /
//! `.btn-secondary` matches manus: those component classes set larger
//! padding/font/radius, and the utilities declared later in `styles.rs` win
//! and shrink the button back down to `h-9 / px-4 py-2 / text-sm`.
//!
//! Per-call overrides (`h-12 text-base`, `bg-blue-600`, …) supplied in `class`
//! continue to win because they are appended after the defaults.

use dioxus::prelude::*;

const DEFAULTS: &str = "inline-flex items-center justify-center gap-2 \
                        whitespace-nowrap rounded-md text-sm font-medium \
                        transition-all bg-primary text-primary-foreground \
                        hover:bg-primary/90 h-9 px-4 py-2";

#[component]
pub fn Button(#[props(default)] class: String, onclick: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
	rsx! {
		button {
			class: "{DEFAULTS} {class}",
			onclick: move |evt| {
				if let Some(h) = &onclick {
					h.call(evt);
				}
			},
			{children}
		}
	}
}
