//! Error capture — the single place that talks to the Sentry JS global.
//!
//! Mirrors landing's `features/error-monitoring/client.ts`: all reporting goes
//! through [`report_error`] so the vendor can be swapped without touching call
//! sites.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Bridge to the global `Sentry` loaded by `boot.rs`. Constructs a real `Error`
// (Sentry drops the stack trace for bare strings) and no-ops while the SDK is
// absent — DSN unset, blocked by an ad-blocker, or not yet loaded.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(
	inline_js = "export function report_to_sentry(message) { if (typeof Sentry !== 'undefined' && Sentry && typeof Sentry.captureException === 'function') { Sentry.captureException(new Error(message)); } }"
)]
extern "C" {
	fn report_to_sentry(message: &str);
}

/// Reports an unexpected error to the monitoring service.
///
/// No-op when the Sentry SDK is not loaded (DSN unset / blocked / not yet
/// initialised), and on non-wasm targets.
pub fn report_error(message: &str) {
	#[cfg(target_arch = "wasm32")]
	report_to_sentry(message);
	#[cfg(not(target_arch = "wasm32"))]
	let _ = message;
}
