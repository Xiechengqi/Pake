mod browser_detect;
mod browser_launch;
mod config;
mod desktop_entry;
mod extension;

use std::process;

fn main() {
    let cfg = config::load_config();

    let browser = match browser_detect::detect_browser() {
        Some(info) => info,
        None => {
            eprintln!(
                "Chrome/Chromium not found. Please install Google Chrome or Chromium, or use Pake without --native flag."
            );
            process::exit(127);
        }
    };

    let status = browser_launch::launch(&browser, &cfg);

    let code = status.code().unwrap_or(1);
    process::exit(code);
}
