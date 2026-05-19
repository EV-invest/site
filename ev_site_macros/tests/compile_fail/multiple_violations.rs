use ev_site_macros::tiered;

fn main() {
	tiered! {
		Block {
			tier: P1,
			A { tier: P1 } //~ ERROR: tier P1 equals enclosing tier; omit `tier:` to inherit from parent
			B { tier: P0 } //~ ERROR: tier P0 must be strictly greater than enclosing tier P1
		}
	};
}
