use std::time::Duration;

use clap::Parser;
pub mod config;
use config::{LiveSettings, SettingsFlags};

#[derive(Default, Parser)]
#[command(author, version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_HASH"), ")"), about, long_about = None)]
struct Cli {
	#[command(flatten)]
	settings: SettingsFlags,
}

fn main() {
	v_utils::clientside!();
	let cli = Cli::parse();
	if let Err(e) = LiveSettings::new(cli.settings, Duration::from_secs(5)) {
		eprintln!("Error reading config: {e}");
		for cause in e.chain().skip(1) {
			eprintln!("  Caused by: {cause}");
		}
		std::process::exit(1);
	}
}
