//! Inline SVG ports of the lucide-react icons used by manus pages.
//!
//! Every icon follows lucide's canonical 24x24 viewBox with `currentColor`
//! strokes, so an enclosing element controls size via `width`/`height` (or
//! `w-*`/`h-*` utility classes) and the icon color via `color`.

use dioxus::prelude::*;

/// Common SVG attributes for every lucide icon.
macro_rules! lucide {
	($name:ident, $($body:tt)*) => {
		#[component]
		pub fn $name(#[props(default = "1.25rem".to_string())] class: String) -> Element {
			rsx! {
				svg {
					class: "{class}",
					xmlns: "http://www.w3.org/2000/svg",
					view_box: "0 0 24 24",
					fill: "none",
					stroke: "currentColor",
					stroke_width: "2",
					stroke_linecap: "round",
					stroke_linejoin: "round",
					$($body)*
				}
			}
		}
	};
}

lucide!(ArrowRight,
	path { d: "M5 12h14" }
	path { d: "m12 5 7 7-7 7" }
);

lucide!(TrendingUp,
	polyline { points: "22 7 13.5 15.5 8.5 10.5 2 17" }
	polyline { points: "16 7 22 7 22 13" }
);

lucide!(Building2,
	path { d: "M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z" }
	path { d: "M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2" }
	path { d: "M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2" }
	path { d: "M10 6h4" }
	path { d: "M10 10h4" }
	path { d: "M10 14h4" }
	path { d: "M10 18h4" }
);

lucide!(
	Zap,
	path {
		d: "M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"
	}
);

lucide!(Globe,
	circle { cx: "12", cy: "12", r: "10" }
	path { d: "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" }
	path { d: "M2 12h20" }
);

lucide!(BarChart3,
	path { d: "M3 3v18h18" }
	path { d: "M18 17V9" }
	path { d: "M13 17V5" }
	path { d: "M8 17v-3" }
);

lucide!(Mail,
	rect { width: "20", height: "16", x: "2", y: "4", rx: "2" }
	path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
);

lucide!(AlertCircle,
	circle { cx: "12", cy: "12", r: "10" }
	line { x1: "12", x2: "12", y1: "8", y2: "12" }
	line { x1: "12", x2: "12.01", y1: "16", y2: "16" }
);

lucide!(Home,
	path { d: "m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
	polyline { points: "9 22 9 12 15 12 15 22" }
);
