use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block {
			tier: P1,
			Child { tier: P1 } //~ ERROR: tier P1 equals enclosing tier; omit `tier:` to inherit from parent
		}
	};
}
