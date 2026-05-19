use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block { } //~ ERROR: root element of `tiered!` must declare `tier: PN`
	};
}
