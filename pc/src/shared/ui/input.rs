use dioxus::prelude::*;

use crate::cn;

#[component]
pub fn Input(
	#[props(default)] class: String,
	#[props(default)] r#type: String,
	#[props(default)] placeholder: String,
	#[props(default)] disabled: bool,
	#[props(default)] value: String,
	oninput: Option<EventHandler<FormEvent>>,
) -> Element {
	let base = "border-input h-9 w-full min-w-0 rounded-md border bg-transparent \
                px-3 py-1 text-sm shadow-xs outline-none transition-colors \
                placeholder:text-muted-foreground \
                disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 \
                focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";

	let cls = cn!(base, class);
	let input_type = if r#type.is_empty() { "text".to_string() } else { r#type };

	rsx! {
		input {
			r#type: input_type,
			class: cls,
			placeholder,
			disabled,
			value,
			oninput: move |e| { if let Some(h) = oninput { h.call(e); } },
		}
	}
}
