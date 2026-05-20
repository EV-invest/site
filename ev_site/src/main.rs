#[cfg(not(target_arch = "wasm32"))]
fn main() -> color_eyre::Result<()> {
	use ev_site::config::{AppConfig, SettingsFlags};
	color_eyre::install()?;
	let cfg = AppConfig::try_build(SettingsFlags::default())?;
	let extra: Vec<String> = std::env::args().skip(1).collect();
	eprintln!("Serving on http://127.0.0.1:{}", cfg.port);
	let status = std::process::Command::new("dx")
		.arg("serve")
		.arg("--package")
		.arg("ev_site")
		.arg("--port")
		.arg(cfg.port.to_string())
		.args(extra)
		.status()?;
	std::process::exit(status.code().unwrap_or(1));
}

#[cfg(target_arch = "wasm32")]
fn main() {
	dioxus::launch(ev_site::App);
}
