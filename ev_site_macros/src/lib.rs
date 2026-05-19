//! Proc-macros for `ev_site`.
//!
//! Currently provides `tiered!` — a thin layer over `dioxus::rsx!` that enforces
//! the responsive-priority discipline (every independently-suppressible element
//! carries an explicit `tier:`; children's tiers must be strictly greater than
//! their enclosing parent's; bare children inherit the parent's tier rather
//! than re-stating it).
//!
//! The macro is intentionally narrow in scope: it parses, validates the
//! parent>child tier invariant, forwards the tree through to `rsx!`
//! unchanged on success, and emits `compile_error!()` at the offending span
//! on violation. All non-tier syntax inside the input is passed through to
//! `rsx!` unchanged.

mod parse;
mod tier;
mod validate;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

use crate::parse::Body;

/// Validates tier invariants in a Dioxus rsx tree, then expands to `rsx!`.
#[proc_macro]
pub fn tiered(input: TokenStream) -> TokenStream {
	let original: TokenStream2 = input.clone().into();
	let body = parse_macro_input!(input as Body);
	if let Some(err) = validate::validate(&body) {
		return err.to_compile_error().into();
	}
	quote! { ::dioxus::prelude::rsx! { #original } }.into()
}
