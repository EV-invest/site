use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block {
			tier: P1,
			A { tier: P1 }
			B { tier: P0 }
		}
	};
}
