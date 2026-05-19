use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block {
			tier: P1,
			Child { tier: P1 }
		}
	};
}
