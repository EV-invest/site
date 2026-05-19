use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block { tier: 1 + 2 } //~ ERROR: expected one of `P0`, `P1`, `P2`, `P3`
	};
}
