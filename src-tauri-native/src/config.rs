use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct NativeBrowserConfig {
    pub url: String,
    pub app_name: String,
    pub width: u32,
    pub height: u32,
}

pub fn load_config() -> NativeBrowserConfig {
    let args: Vec<String> = env::args().collect();

    // Parse CLI args: pake-native <url> [--name <name>] [--width <w>] [--height <h>]
    let cli_url = args.get(1).filter(|a| !a.starts_with("--")).cloned();

    let cli_name = find_arg(&args, "--name");
    let cli_width = find_arg(&args, "--width").and_then(|v| v.parse().ok());
    let cli_height = find_arg(&args, "--height").and_then(|v| v.parse().ok());

    // Embedded defaults from compile time
    let defaults: NativeBrowserConfig =
        serde_json::from_str(include_str!("../.pake/native_config.json"))
            .expect("Failed to parse native_config.json");

    // CLI args override embedded defaults
    let url = cli_url.unwrap_or(defaults.url);
    let app_name = cli_name.unwrap_or(defaults.app_name);
    let width = cli_width.unwrap_or(defaults.width);
    let height = cli_height.unwrap_or(defaults.height);

    if url == "https://example.com" || url.is_empty() {
        print_usage();
        std::process::exit(1);
    }

    NativeBrowserConfig { url, app_name, width, height }
}

fn find_arg(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn print_usage() {
    eprintln!("Usage: pake <url> [options]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --name <string>    Application name (default: Pake)");
    eprintln!("  --width <number>   Window width (default: 1200)");
    eprintln!("  --height <number>  Window height (default: 780)");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  pake https://x.com/ --name X");
}
