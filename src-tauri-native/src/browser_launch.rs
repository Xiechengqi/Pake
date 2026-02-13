use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};

use crate::browser_detect::BrowserInfo;
use crate::config::NativeBrowserConfig;
use crate::extension;

fn get_data_dir(config: &NativeBrowserConfig, browser: &BrowserInfo) -> PathBuf {
    let config_dir = dirs::config_dir().expect("Failed to resolve config directory");
    let logical_dir = config_dir.join(&config.app_name).join(&browser.name);

    if browser.is_snap {
        // Snap browsers can only write to ~/snap/<name>/common/
        // Create the real dir there, then symlink from the logical path
        let home = dirs::home_dir().expect("Failed to resolve home directory for snap data dir");
        let snap_dir = home
            .join("snap")
            .join(&browser.name)
            .join("common")
            .join("pake")
            .join(&config.app_name);
        std::fs::create_dir_all(&snap_dir).expect("Failed to create snap data directory");

        // Create symlink: ~/.config/<app>/<browser> -> ~/snap/<browser>/common/pake/<app>
        let parent = logical_dir.parent().unwrap();
        std::fs::create_dir_all(parent).expect("Failed to create config directory");
        if !logical_dir.exists() {
            #[cfg(unix)]
            std::os::unix::fs::symlink(&snap_dir, &logical_dir)
                .unwrap_or_else(|e| eprintln!("Warning: failed to create symlink: {}", e));
        }

        return snap_dir;
    }

    logical_dir
}

pub fn launch(browser: &BrowserInfo, config: &NativeBrowserConfig) -> ExitStatus {
    let data_dir = get_data_dir(config, browser);
    std::fs::create_dir_all(&data_dir).expect("Failed to create browser data directory");

    let window_size = format!("--window-size={},{}", config.width, config.height);

    let mut cmd = Command::new(&browser.path);
    cmd.arg(format!("--app={}", config.url))
        .arg(format!("--user-data-dir={}", data_dir.display()))
        .arg(&window_size)
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--disable-features=MediaRouter")
        .arg("--disable-background-networking")
        .arg("--disable-logging")
        .arg("--log-level=3")
        .arg(format!(
            "--load-extension={}",
            extension::ensure_extension(&data_dir).display()
        ));

    // Running as root on Linux requires --no-sandbox
    // Auto-detect Wayland vs X11 display server
    #[cfg(target_os = "linux")]
    {
        let is_root = std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("Uid:"))
                    .map(|l| l.split_whitespace().nth(1) == Some("0"))
            })
            .unwrap_or(false);
        if is_root {
            cmd.arg("--no-sandbox");
        }

        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            cmd.arg("--ozone-platform=wayland");
        }
    }

    let mut child = cmd
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to launch {}: {}", browser.name, e));

    child.wait().unwrap_or_else(|e| panic!("Failed to wait for {} process: {}", browser.name, e))
}
