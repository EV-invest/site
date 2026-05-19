use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block { tier: P9 } //~ ERROR: expected one of `P0`, `P1`, `P2`, `P3`
	};
}
