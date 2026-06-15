//! Boots client-side error monitoring — mirrors landing's `provider.tsx`.
//!
//! The native `sentry` Rust crate has no browser-wasm transport, so the Sentry
//! **JavaScript Browser SDK** is loaded into the page from a pinned CDN bundle
//! and initialised once it loads. Rust panics are forwarded to it via a panic
//! hook. A complete no-op when `CABINET_SENTRY_DSN` is unset.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Public (browser) Sentry DSN, baked at compile time — safe to ship in the
/// client bundle, exactly like landing's `NEXT_PUBLIC_SENTRY_DSN`. Unset (or
/// empty) disables monitoring. `option_env!` reads it at compile time, so a
/// changed value needs a rebuild (`cargo clean -p cabinet`) to take effect.
#[cfg(target_arch = "wasm32")]
const SENTRY_DSN: Option<&str> = option_env!("CABINET_SENTRY_DSN");

/// Deployment environment tag shown in Sentry. Mirrors `NEXT_PUBLIC_APP_ENV`.
#[cfg(target_arch = "wasm32")]
const APP_ENV: &str = match option_env!("CABINET_APP_ENV") {
	Some(env) => env,
	None => "development",
};

/// Pinned Sentry Browser SDK CDN bundle. The `tracing` variant captures errors
/// and light performance traces with **no Session Replay** — cabinet renders
/// client financial data, which replay would record. Bump deliberately.
#[cfg(target_arch = "wasm32")]
const SENTRY_SDK_URL: &str = "https://browser.sentry-cdn.com/10.57.0/bundle.tracing.min.js";

// Injects the SDK <script> and initialises Sentry in its `onload` (Dioxus head
// injection is one-shot and gives no load hook, so this self-manages ordering).
// `tracesSampleRate: 0.1` matches landing's production tracing budget.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(
	inline_js = "export function boot_sentry(sdkUrl, dsn, environment) { if (!dsn || window.__sentryBooted) return; window.__sentryBooted = true; var s = document.createElement('script'); s.src = sdkUrl; s.crossOrigin = 'anonymous'; s.onload = function () { if (!window.Sentry) return; Sentry.init({ dsn: dsn, environment: environment, integrations: [Sentry.browserTracingIntegration()], tracesSampleRate: 0.1 }); }; document.head.appendChild(s); }"
)]
extern "C" {
	fn boot_sentry(sdk_url: &str, dsn: &str, environment: &str);
}

/// Boots error monitoring. Call once from the `App` component (via `use_hook`),
/// NOT from `main` before launch: Dioxus installs its own panic hook from inside
/// its async runtime (after `main` returns), so installing earlier would be
/// clobbered — running on first render lets our hook chain over Dioxus's instead.
///
/// Injects the Sentry SDK and installs a panic hook that forwards Rust panics
/// to [`super::capture::report_error`]. No-op when the DSN is unset or empty.
pub fn init() {
	#[cfg(target_arch = "wasm32")]
	if let Some(dsn) = SENTRY_DSN.filter(|dsn| !dsn.is_empty()) {
		boot_sentry(SENTRY_SDK_URL, dsn, APP_ENV);
		install_panic_hook();
	}
}

/// Installs a Sentry-forwarding panic hook, chaining the hook already in place
/// (the std default, or Dioxus's devtools console hook in debug builds) so local
/// console output is preserved. Default wasm `panic = "abort"` means Sentry's own
/// JS global error handlers never see Rust panics — this hook is the only path.
#[cfg(target_arch = "wasm32")]
fn install_panic_hook() {
	use std::sync::Once;
	static HOOK: Once = Once::new();
	HOOK.call_once(|| {
		let prev = std::panic::take_hook();
		std::panic::set_hook(Box::new(move |info| {
			super::capture::report_error(&info.to_string());
			prev(info);
		}));
	});
}
