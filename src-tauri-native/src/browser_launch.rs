use std::path::PathBuf;
use std::process::{Command, ExitStatus};

use crate::browser_detect::BrowserInfo;
use crate::config::NativeBrowserConfig;

fn get_data_dir(config: &NativeBrowserConfig, browser_name: &str) -> PathBuf {
    let config_dir = dirs::config_dir().expect("Failed to resolve config directory");
    config_dir.join(&config.app_name).join(browser_name)
}

pub fn launch(browser: &BrowserInfo, config: &NativeBrowserConfig) -> ExitStatus {
    let data_dir = get_data_dir(config, &browser.name);
    std::fs::create_dir_all(&data_dir).expect("Failed to create browser data directory");

    let window_size = format!("--window-size={},{}", config.width, config.height);

    let mut child = Command::new(&browser.path)
        .arg(format!("--app={}", config.url))
        .arg(format!("--user-data-dir={}", data_dir.display()))
        .arg(&window_size)
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--disable-extensions")
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to launch {}: {}", browser.name, e));

    child.wait().unwrap_or_else(|e| panic!("Failed to wait for {} process: {}", browser.name, e))
}
