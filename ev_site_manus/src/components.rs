//! Shared building blocks for the manus port.
//!
//! Thin ports of shadcn-ui primitives (`Button`, `Card`, `CardContent`) and
//! semantic wrappers extracted from the repeated markup in `pages/home.rs` and
//! `pages/not_found.rs`. Each component preserves the manus class strings
//! verbatim so the cascade interaction with the bespoke `.btn-primary` /
//! `.card-minimal` rules in `styles.rs` matches the original site.

use dioxus::prelude::*;

use crate::icons::Building2;

const BTN_DEFAULTS: &str = "inline-flex items-center justify-center gap-2 \
                            whitespace-nowrap rounded-md text-sm font-medium \
                            transition-all bg-primary text-primary-foreground \
                            hover:bg-primary/90 h-9 px-4 py-2";

/// Port of shadcn-ui's `Button` (manus/client/src/components/ui/button.tsx)
/// restricted to the `default` variant/size — the only configuration used.
/// Per-call overrides supplied in `class` win because they are appended after
/// the defaults.
#[component]
pub fn Button(#[props(default)] class: String, onclick: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
	rsx! {
		button {
			class: "{BTN_DEFAULTS} {class}",
			onclick: move |evt| {
				if let Some(h) = &onclick {
					h.call(evt);
				}
			},
			{children}
		}
	}
}

/// Centered title + subtitle pair used above every non-hero section.
#[component]
pub fn SectionHeader(title: String, description: String) -> Element {
	rsx! {
		div { class: "text-center mb-16",
			h2 { class: "text-4xl md:text-5xl font-bold mb-4", "{title}" }
			p { class: "text-lg text-muted-foreground max-w-2xl mx-auto", "{description}" }
		}
	}
}

/// Gradient square logo + "RealEstate Fund" wordmark. Header uses the default
/// `font-bold text-lg tracking-tight`; footer overrides with plain `font-bold`.
#[component]
pub fn BrandMark(#[props(default)] text_class: Option<String>) -> Element {
	let span_class = match text_class {
		Some(c) => c,
		None => "font-bold text-lg tracking-tight".to_string(),
	};
	rsx! {
		div { class: "flex items-center gap-2",
			div {
				class: "w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-accent flex items-center justify-center",
				Building2 { class: "w-5 h-5 text-white" }
			}
			span { class: "{span_class}", "RealEstate Fund" }
		}
	}
}

/// Generic page section wrapper: `<section>` with `relative py-24 md:py-32`,
/// optional absolute bg-image, optional overlay, plus the `container max-w-{N}`
/// content wrapper. The Hero section is intentionally not routed through here
/// because it needs `min-h-screen flex items-center` and fixed bg attachment.
#[component]
pub fn Section(
	#[props(default)] id: Option<String>,
	#[props(default)] bg_image: Option<&'static str>,
	#[props(default)] bg_opacity: Option<f32>,
	#[props(default)] overlay_class: Option<String>,
	#[props(default)] extra_class: Option<String>,
	max_width: String,
	children: Element,
) -> Element {
	let section_class = match extra_class {
		Some(extra) => format!("relative py-24 md:py-32 {extra}"),
		None => "relative py-24 md:py-32".to_string(),
	};
	let wrapper_class = format!("relative z-10 container max-w-{max_width}");
	let bg_style = bg_image.map(|img| {
		let opacity = match bg_opacity {
			Some(o) => format!(" opacity: {o};"),
			None => String::new(),
		};
		format!("background-image: {img}; background-size: cover; background-position: center;{opacity}")
	});
	let has_bg = bg_image.is_some();
	rsx! {
		section {
			id,
			class: "{section_class}",
			if let Some(style) = bg_style {
				div { class: "absolute inset-0 z-0", style: "{style}" }
			}
			if let Some(overlay) = overlay_class {
				// When a bg image sits at z-0, the overlay needs to land
				// between it and the z-10 content wrapper — manus encodes
				// that with an inline `z-index: 5` since Tailwind has no
				// `z-5` utility. Without a bg image the overlay is just a
				// background fill and the caller's class controls z-index.
				if has_bg {
					div { class: "{overlay}", style: "z-index: 5;" }
				} else {
					div { class: "{overlay}" }
				}
			}
			div { class: "{wrapper_class}",
				{children}
			}
		}
	}
}

/// Port of shadcn-ui's `Card` (manus/client/src/components/ui/card.tsx)
/// restricted to the slots actually used (`Card`, `CardContent`).
#[component]
pub fn Card(#[props(default)] class: String, children: Element) -> Element {
	let full = format!("bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm {class}");
	rsx! {
		div { class: "{full}", {children} }
	}
}

#[component]
pub fn CardContent(#[props(default)] class: String, children: Element) -> Element {
	let full = format!("px-6 {class}");
	rsx! {
		div { class: "{full}", {children} }
	}
}

/// Pill-style label used for the hero eyebrow.
#[component]
pub fn Badge(#[props(default)] class: Option<String>, children: Element) -> Element {
	let extra = match class {
		Some(c) => c,
		None => String::new(),
	};
	let full = format!("inline-block px-4 py-2 rounded-full border border-primary/30 bg-primary/5 backdrop-blur-sm {extra}");
	rsx! {
		div { class: "{full}", {children} }
	}
}

/// Centered headline + primary/secondary buttons + optional disclaimer used by
/// the Contact section. Sits inside a `Section`; wraps its own content in a
/// `text-center` div so the surrounding `Section` keeps its plain wrapper.
#[component]
pub fn SectionCTA(title: String, description: String, primary_label: String, primary_icon: Element, secondary_label: String, #[props(default)] disclaimer: Option<String>) -> Element {
	rsx! {
		div { class: "text-center",
			h2 { class: "text-4xl md:text-5xl font-bold mb-6", "{title}" }
			p { class: "text-lg text-muted-foreground mb-8 max-w-xl mx-auto", "{description}" }
			div { class: "flex flex-col sm:flex-row gap-4 justify-center",
				Button { class: "btn-primary gap-2 h-12 text-base",
					"{primary_label}"
					{primary_icon}
				}
				Button { class: "btn-secondary h-12 text-base", "{secondary_label}" }
			}
			if let Some(text) = disclaimer {
				div { class: "mt-12 pt-12 border-t border-border",
					p { class: "text-xs text-muted-foreground", "{text}" }
				}
			}
		}
	}
}
