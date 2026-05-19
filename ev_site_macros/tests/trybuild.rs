//! Compile-fail tests for the tier validator.
//!
//! Pass-side coverage lives as unit tests inside the crate (no need to drag
//! `dioxus` into the test compilation just to make `rsx!` resolve in the
//! macro's success path). Fail cases emit only `compile_error!`s, so they're
//! testable in isolation.

#[test]
fn compile_fail() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/fail/*.rs");
}
