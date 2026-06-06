use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Success,
}

#[component]
pub fn Badge(
    #[props(default)] variant: BadgeVariant,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let base = "inline-flex items-center justify-center rounded-md border \
                px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap";

    let variant_cls = match variant {
        BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground",
        BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground",
        BadgeVariant::Destructive => "border-transparent bg-destructive text-white",
        BadgeVariant::Outline => "text-foreground",
        BadgeVariant::Success => "border-transparent bg-main-accent-t2/20 text-main-accent-t2",
    };

    let cls = format!("{base} {variant_cls} {class}");

    rsx! {
        span { class: cls, {children} }
    }
}
