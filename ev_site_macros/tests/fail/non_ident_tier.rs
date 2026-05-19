use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block { tier: 1 + 2 }
	};
}
