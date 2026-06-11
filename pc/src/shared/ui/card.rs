use dioxus::prelude::*;

#[component]
pub fn Card(#[props(default)] class: String, children: Element) -> Element {
	let cls = format!("bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm {class}");
	rsx! {
		div { class: cls, {children} }
	}
}

#[component]
pub fn CardHeader(#[props(default)] class: String, children: Element) -> Element {
	let cls = format!("grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 {class}");
	rsx! {
		div { class: cls, {children} }
	}
}

#[component]
pub fn CardTitle(#[props(default)] class: String, children: Element) -> Element {
	let cls = format!("leading-none font-semibold text-foreground {class}");
	rsx! {
		div { class: cls, {children} }
	}
}

#[component]
pub fn CardDescription(#[props(default)] class: String, children: Element) -> Element {
	let cls = format!("text-muted-foreground text-sm {class}");
	rsx! {
		div { class: cls, {children} }
	}
}

#[component]
pub fn CardAction(#[props(default)] class: String, children: Element) -> Element {
	let cls = format!("col-start-2 row-span-2 row-start-1 self-start justify-self-end {class}");
	rsx! {
		div { class: cls, {children} }
	}
}

#[component]
pub fn CardContent(#[props(default)] class: String, children: Element) -> Element {
	let cls = format!("px-6 {class}");
	rsx! {
		div { class: cls, {children} }
	}
}

#[component]
pub fn CardFooter(#[props(default)] class: String, children: Element) -> Element {
	let cls = format!("flex items-center px-6 {class}");
	rsx! {
		div { class: cls, {children} }
	}
}
