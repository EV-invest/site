//! Tier identifier recogniser.
//!
//! The macro accepts a closed set of tier idents — `P0`, `P1`, `P2`, `P3` —
//! supplied as the value of a `tier:` attribute. Anything else is a hard
//! error surfaced at the offending expression span.

use syn::{Expr, ExprPath, spanned::Spanned};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum TierIdent {
	P0,
	P1,
	P2,
	P3,
}

impl TierIdent {
	pub fn ordinal(self) -> u8 {
		match self {
			Self::P0 => 0,
			Self::P1 => 1,
			Self::P2 => 2,
			Self::P3 => 3,
		}
	}

	/// Parse a `tier: …` attribute value. Only a bare path of one of the four
	/// recognised idents is accepted.
	pub fn from_expr(expr: &Expr) -> Result<Self, syn::Error> {
		let path = match expr {
			Expr::Path(ExprPath { path, qself: None, attrs }) if attrs.is_empty() => path,
			_ => return Err(syn::Error::new(expr.span(), "expected one of `P0`, `P1`, `P2`, `P3`")),
		};
		let Some(ident) = path.get_ident() else {
			return Err(syn::Error::new(expr.span(), "expected one of `P0`, `P1`, `P2`, `P3`"));
		};
		match ident.to_string().as_str() {
			"P0" => Ok(Self::P0),
			"P1" => Ok(Self::P1),
			"P2" => Ok(Self::P2),
			"P3" => Ok(Self::P3),
			_ => Err(syn::Error::new(ident.span(), "expected one of `P0`, `P1`, `P2`, `P3`")),
		}
	}
}

impl std::fmt::Display for TierIdent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "P{}", self.ordinal())
	}
}
