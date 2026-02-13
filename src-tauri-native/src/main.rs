mod browser_detect;
mod browser_launch;
mod config;

use std::process;

fn main() {
    let cfg = config::load_config();

    let chrome_path = match browser_detect::detect_chrome() {
        Some(path) => path,
        None => {
            eprintln!(
                "Chrome not found. Please install Google Chrome, or use Pake without --native flag."
            );
            process::exit(127);
        }
    };

    let status = browser_launch::launch(&chrome_path, &cfg);

    let code = status.code().unwrap_or(1);
    process::exit(code);
}
