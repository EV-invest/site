#[cfg(not(target_arch = "wasm32"))]
fn main() {
	let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(50737);
	eprintln!("Serving on http://127.0.0.1:{port}");
	let extra: Vec<String> = std::env::args().skip(1).collect();
	let status = std::process::Command::new("dx")
		.arg("serve")
		.arg("--package")
		.arg("ev_site_manus")
		.arg("--port")
		.arg(port.to_string())
		.args(extra)
		.status()
		.expect("dx serve failed to start");
	std::process::exit(status.code().unwrap_or(1));
}

#[cfg(target_arch = "wasm32")]
fn main() {
	dioxus::launch(ev_site_manus::App);
}
