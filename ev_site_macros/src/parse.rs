//! Shallow parser over a Dioxus `rsx!`-shaped token tree.
//!
//! The grammar we care about is intentionally narrow:
//!
//! - element       := `Path { body }`
//! - body item     := attribute | child
//! - attribute     := `ident : Expr`
//! - child         := element | text literal | `{ … }` block | unknown tokens
//!
//! Anything we don't explicitly recognise is captured verbatim into an
//! `Opaque` variant so the input can be forwarded back to `rsx!` unchanged on
//! success. The macro is a *validator*, not a re-emitter, and must not
//! constrain rsx syntax that's outside the tier discipline.

use proc_macro2::{Span, TokenStream, TokenTree};
use syn::{
	Expr, Ident, Path, Token,
	parse::{Parse, ParseStream},
	spanned::Spanned,
	token,
};


pub struct Body {
	pub items: Vec<BodyItem>,
}

pub enum BodyItem {
	Attr { name: Ident, value: Expr },
	Child(Node),
}

pub enum Node {
	Element { body: Body, name_span: Span },
	Opaque,
}

impl Parse for Body {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut items = Vec::new();
		while !input.is_empty() {
			items.push(parse_body_item(input)?);
			// rsx tolerates optional commas between body items.
			let _ = input.parse::<Token![,]>();
		}
		Ok(Self { items })
	}
}

fn parse_body_item(input: ParseStream) -> syn::Result<BodyItem> {
	// (1) attribute: `ident :` — single colon, not `::`.
	if input.peek(Ident) && input.peek2(Token![:]) && !input.peek2(Token![::]) {
		let name: Ident = input.parse()?;
		input.parse::<Token![:]>()?;
		let value: Expr = input.parse()?;
		return Ok(BodyItem::Attr { name, value });
	}

	// (2) text literal child.
	if input.peek(syn::LitStr) {
		input.parse::<syn::LitStr>()?;
		return Ok(BodyItem::Child(Node::Opaque));
	}

	// (3) brace-block child (`{ expr }` interpolation).
	if input.peek(token::Brace) {
		let inner;
		let _ = syn::braced!(inner in input);
		inner.parse::<TokenStream>()?;
		return Ok(BodyItem::Child(Node::Opaque));
	}

	// (4) element: `Path { body }`. Fork to confirm the brace before
	// committing — keeps unknown leading paths out of element-parsing.
	{
		let fork = input.fork();
		if fork.parse::<Path>().is_ok() && fork.peek(token::Brace) {
			let path: Path = input.parse()?;
			let content;
			let _ = syn::braced!(content in input);
			let body: Body = content.parse()?;
			let name_span = path.span();
			return Ok(BodyItem::Child(Node::Element { body, name_span }));
		}
	}

	// (5) anything else: eat one logical token tree as opaque. Brace groups
	// are atomic at the proc-macro level, so commas inside nested braces
	// don't leak out.
	input.parse::<TokenTree>()?;
	Ok(BodyItem::Child(Node::Opaque))
}

/// Iterate the element nodes of a body (skipping attributes and opaque children).
pub fn elements(body: &Body) -> impl Iterator<Item = &Node> {
	body.items.iter().filter_map(|it| match it {
		BodyItem::Child(node @ Node::Element { .. }) => Some(node),
		_ => None,
	})
}

/// Look up the `tier:` attribute on a body, if present.
pub fn find_tier_attr(body: &Body) -> Option<&Expr> {
	body.items.iter().find_map(|it| match it {
		BodyItem::Attr { name, value } if name == "tier" => Some(value),
		_ => None,
	})
}
