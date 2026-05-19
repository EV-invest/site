use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block {
			tier: P2,
			Child { tier: P0 }
		}
	};
}
