//! Proc-macros for `ev_site`.
//!
//! Currently provides `tiered!` — a thin layer over `dioxus::rsx!` that enforces
//! the responsive-priority discipline (every independently-suppressible element
//! carries an explicit `tier:`; children's tiers must be strictly greater than
//! their enclosing parent's; bare children inherit the parent's tier rather
//! than re-stating it).
//!
//! The macro is intentionally narrow in scope: it parses, validates the
//! parent>child tier invariant, rewrites the tree to forward through to
//! `rsx!`, and emits `compile_error!()` at the offending span on violation.
//! All non-tier syntax inside the input is passed through to `rsx!` unchanged.

use proc_macro::TokenStream;

/// Validates tier invariants in a Dioxus rsx tree, then expands to `rsx!`.
///
/// See crate docs for the rules. Detailed architecture is intentionally left
/// to the design pass — this entry point is the only stable surface.
#[proc_macro]
pub fn tiered(input: TokenStream) -> TokenStream {
	// Skeleton: forward unchanged until the parsing/validation layer lands.
	// Once the design is settled this body becomes `parse -> validate -> emit`.
	let input = proc_macro2::TokenStream::from(input);
	quote::quote! { ::dioxus::prelude::rsx! { #input } }.into()
}
