//! Tier invariant checker.
//!
//! Walks the shallow AST produced by [`crate::parse`] and verifies that, for
//! every element carrying an explicit `tier: PN`:
//!
//! - the root of each top-level element declares one (mandatory);
//! - a child's tier is strictly greater than the nearest enclosing tier;
//! - equal-or-less tiers are a hard error.
//!
//! Elements without a `tier:` attribute inherit from the nearest enclosing
//! one — they don't need to restate it.

use syn::spanned::Spanned;

use crate::{
	parse::{Body, Node, elements, find_tier_attr},
	tier::TierIdent,
};

/// Run validation across all top-level elements of the macro input. Returns
/// the combined error if any rule was violated.
pub fn validate(body: &Body) -> Option<syn::Error> {
	let mut acc: Option<syn::Error> = None;
	for node in elements(body) {
		walk(node, None, &mut acc);
	}
	acc
}

fn walk(node: &Node, parent: Option<TierIdent>, acc: &mut Option<syn::Error>) {
	let Node::Element { body, name_span, .. } = node else { return };

	let tier_expr = find_tier_attr(body);
	let own = match tier_expr {
		Some(expr) => match TierIdent::from_expr(expr) {
			Ok(t) => Some(t),
			Err(e) => {
				push(acc, e);
				// Malformed tier value: skip parent>child checks but still
				// recurse with the enclosing tier so deeper issues surface.
				for child in elements(body) {
					walk(child, parent, acc);
				}
				return;
			}
		},
		None => None,
	};

	let descend_with = match (parent, own) {
		(None, None) => {
			push(acc, syn::Error::new(*name_span, "root element of `tiered!` must declare `tier: PN`"));
			None
		}
		(None, Some(t)) => Some(t),
		(Some(p), None) => Some(p),
		(Some(p), Some(c)) => {
			let span = tier_expr.map(Spanned::span).unwrap_or(*name_span);
			if c == p {
				push(acc, syn::Error::new(span, format!("tier {c} equals enclosing tier; omit `tier:` to inherit from parent")));
				Some(p)
			} else if c < p {
				push(acc, syn::Error::new(span, format!("tier {c} must be strictly greater than enclosing tier {p}")));
				Some(p)
			} else {
				Some(c)
			}
		}
	};

	for child in elements(body) {
		walk(child, descend_with, acc);
	}
}

fn push(acc: &mut Option<syn::Error>, err: syn::Error) {
	match acc {
		Some(existing) => existing.combine(err),
		slot @ None => *slot = Some(err),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn ok(src: &str) {
		let body: Body = syn::parse_str(src).expect("body parses");
		assert!(validate(&body).is_none(), "expected pass, got error for: {src}");
	}

	fn err(src: &str) -> String {
		let body: Body = syn::parse_str(src).expect("body parses");
		validate(&body).expect("expected error").to_string()
	}

	#[test]
	fn root_only() {
		ok("Block { tier: P0 }");
	}

	#[test]
	fn strict_increase() {
		ok("A { tier: P0, B { tier: P1, C { tier: P2, D { tier: P3 } } } }");
	}

	#[test]
	fn inheritance_passes_through() {
		ok("A { tier: P0, div { span { p { \"hi\" } } } }");
	}

	#[test]
	fn mixed_attrs() {
		ok(r#"A { tier: P0, class: "c", style: "s", B { tier: P1, style: "x" } }"#);
	}

	#[test]
	fn turbofish_component() {
		ok("Outer { tier: P0, Outlet::<Route> { tier: P1 } }");
	}

	#[test]
	fn expression_children() {
		ok(r#"A { tier: P0, "text" { value } }"#);
	}

	#[test]
	fn struct_literal_attr_value() {
		ok("A { tier: P0, Link { tier: P1, to: Route::Home {} } }");
	}

	#[test]
	fn lowercase_html_tags() {
		ok("div { tier: P0, span { p { \"x\" } } }");
	}

	#[test]
	fn missing_root_tier() {
		assert!(err("Block { }").contains("root"));
	}

	#[test]
	fn equal_to_parent() {
		let e = err("A { tier: P1, B { tier: P1 } }");
		assert!(e.contains("equals"), "got: {e}");
	}

	#[test]
	fn lesser_than_parent() {
		let e = err("A { tier: P2, B { tier: P0 } }");
		assert!(e.contains("strictly greater"), "got: {e}");
	}

	#[test]
	fn unknown_tier_ident() {
		let e = err("A { tier: P9 }");
		assert!(e.contains("P0"), "got: {e}");
	}

	#[test]
	fn non_ident_tier() {
		let e = err("A { tier: 1 + 2 }");
		assert!(e.contains("P0"), "got: {e}");
	}

	#[test]
	fn multiple_violations_combined() {
		let body: Body = syn::parse_str("A { tier: P1, B { tier: P1, C { tier: P0 } } }").unwrap();
		let combined = validate(&body).expect("errors");
		// Two distinct violations should be combined under one Error.
		assert_eq!(combined.into_iter().count(), 2);
	}
}
