use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use crate::config::NativeBrowserConfig;

fn get_data_dir(config: &NativeBrowserConfig) -> PathBuf {
    let config_dir = dirs::config_dir().expect("Failed to resolve config directory");
    config_dir.join(&config.app_name).join("chrome")
}

pub fn launch(chrome_path: &Path, config: &NativeBrowserConfig) -> ExitStatus {
    let data_dir = get_data_dir(config);
    std::fs::create_dir_all(&data_dir).expect("Failed to create chrome data directory");

    let window_size = format!("--window-size={},{}", config.width, config.height);

    let mut child = Command::new(chrome_path)
        .arg(format!("--app={}", config.url))
        .arg(format!("--user-data-dir={}", data_dir.display()))
        .arg(&window_size)
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--disable-extensions")
        .spawn()
        .expect("Failed to launch Chrome");

    child.wait().expect("Failed to wait for Chrome process")
}
