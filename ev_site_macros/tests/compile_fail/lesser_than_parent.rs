use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block {
			tier: P2,
			Child { tier: P0 } //~ ERROR: tier P0 must be strictly greater than enclosing tier P2
		}
	};
}
