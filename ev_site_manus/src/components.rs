//! Manus-specific layout wrappers. Primitive components (Button, Card, Badge,
//! …) come from `shadcn_ui`; everything here is bespoke to the manus page
//! structure.

use dioxus::prelude::*;
use lucide_dioxus::Building2;
use shadcn_ui::{Button, ButtonSize, ButtonVariant};

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

/// Gradient square logo + "RealEstate Fund" wordmark. Required `text_class` —
/// no silent default; header passes the bold/tracking-tight string, footer
/// overrides with plain `font-bold`.
#[component]
pub fn BrandMark(text_class: String) -> Element {
	rsx! {
		div { class: "flex items-center gap-2",
			div {
				class: "w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-accent flex items-center justify-center",
				Building2 { class: "w-5 h-5 text-white" }
			}
			span { class: "{text_class}", "RealEstate Fund" }
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
	#[props(default)] bg_image: Option<String>,
	#[props(default)] bg_opacity: Option<f32>,
	#[props(default)] overlay_class: Option<String>,
	#[props(default)] extra_class: Option<String>,
	max_width: String,
	children: Element,
) -> Element {
	let bg_style = bg_image.as_ref().map(|img| {
		let opacity = bg_opacity.map(|o| format!(" opacity: {o};")).unwrap_or_default();
		format!("background-image: {img}; background-size: cover; background-position: center;{opacity}")
	});
	let has_bg = bg_image.is_some();
	let wrapper_class = format!("relative z-10 container max-w-{max_width}");
	let section_class = match &extra_class {
		Some(extra) => format!("relative py-24 md:py-32 {extra}"),
		None => "relative py-24 md:py-32".to_string(),
	};
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
				Button {
					variant: ButtonVariant::Default,
					size: ButtonSize::Lg,
					class: "text-base",
					"{primary_label}"
					{primary_icon}
				}
				Button {
					variant: ButtonVariant::Outline,
					size: ButtonSize::Lg,
					class: "text-base",
					"{secondary_label}"
				}
			}
			if let Some(text) = disclaimer {
				div { class: "mt-12 pt-12 border-t border-border",
					p { class: "text-xs text-muted-foreground", "{text}" }
				}
			}
		}
	}
}
