//! Client-side error capture — single entry point for error reporting.
//!
//! Mirrors landing's `features/error-monitoring`. Because the native `sentry`
//! Rust crate has no browser-wasm transport, the Sentry **JavaScript Browser
//! SDK** is loaded into the page and Rust talks to it through a thin
//! wasm-bindgen bridge.
//!
//! **Public API**
//! - [`init`] — called once from the `App` component (`use_hook`); boots the SDK
//!   and installs a panic hook. No-op when `CABINET_SENTRY_DSN` is unset.
//! - [`report_error`] — capture an unexpected error from anywhere in the app.
//!
//! **Rule:** never touch the JS `Sentry` global outside this slice — init and
//! capture live here so the vendor can be swapped without touching call sites.

mod boot;
mod capture;

pub use boot::init;
pub use capture::report_error;
