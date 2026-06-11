use dioxus::prelude::*;

use crate::cn;

#[component]
pub fn Table(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!("w-full caption-bottom text-sm", class);
	rsx! {
		div { class: "relative w-full overflow-x-auto",
			table { class: cls, {children} }
		}
	}
}

#[component]
pub fn TableHeader(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!("[&_tr]:border-b", class);
	rsx! {
		thead { class: cls, {children} }
	}
}

#[component]
pub fn TableBody(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!("[&_tr:last-child]:border-0", class);
	rsx! {
		tbody { class: cls, {children} }
	}
}

#[component]
pub fn TableRow(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!("border-b transition-colors hover:bg-muted/50", class);
	rsx! {
		tr { class: cls, {children} }
	}
}

#[component]
pub fn TableHead(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(
		"text-muted-foreground h-10 px-3 text-left align-middle text-xs font-medium \
         uppercase tracking-wide whitespace-nowrap",
		class
	);
	rsx! {
		th { class: cls, {children} }
	}
}

#[component]
pub fn TableCell(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!("px-3 py-3 align-middle whitespace-nowrap", class);
	rsx! {
		td { class: cls, {children} }
	}
}

#[component]
pub fn TableCaption(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!("text-muted-foreground mt-4 text-sm", class);
	rsx! {
		caption { class: cls, {children} }
	}
}
