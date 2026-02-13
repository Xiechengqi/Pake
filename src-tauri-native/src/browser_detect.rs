use std::path::PathBuf;

pub struct BrowserInfo {
    /// "chrome" or "chromium", used as data directory name
    pub name: String,
    pub path: PathBuf,
    /// Whether the browser is installed via snap (Linux only)
    pub is_snap: bool,
}

#[cfg(target_os = "macos")]
pub fn detect_browser() -> Option<BrowserInfo> {
    let candidates = [
        ("chrome", "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"),
        ("chromium", "/Applications/Chromium.app/Contents/MacOS/Chromium"),
    ];
    for (name, path_str) in &candidates {
        let path = PathBuf::from(path_str);
        if path.exists() {
            return Some(BrowserInfo { name: name.to_string(), path, is_snap: false });
        }
    }
    // Also check ~/Applications
    if let Some(home) = dirs::home_dir() {
        let user_apps = home.join("Applications");
        let user_candidates = [
            ("chrome", user_apps.join("Google Chrome.app/Contents/MacOS/Google Chrome")),
            ("chromium", user_apps.join("Chromium.app/Contents/MacOS/Chromium")),
        ];
        for (name, path) in &user_candidates {
            if path.exists() {
                return Some(BrowserInfo { name: name.to_string(), path: path.clone(), is_snap: false });
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
pub fn detect_browser() -> Option<BrowserInfo> {
    // Chrome via registry
    if let Ok(hklm) = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
        .open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe")
    {
        if let Ok(path) = hklm.get_value::<String, _>("") {
            let p = PathBuf::from(&path);
            if p.exists() {
                return Some(BrowserInfo { name: "chrome".to_string(), path: p, is_snap: false });
            }
        }
    }

    // Common paths: Chrome then Chromium
    let program_dirs = [
        std::env::var("ProgramFiles").unwrap_or_default(),
        std::env::var("ProgramFiles(x86)").unwrap_or_default(),
        std::env::var("LocalAppData").unwrap_or_default(),
    ];
    let exe_candidates = [
        ("chrome", "Google\\Chrome\\Application\\chrome.exe"),
        ("chromium", "Chromium\\Application\\chrome.exe"),
    ];
    for (name, rel) in &exe_candidates {
        for base in &program_dirs {
            if base.is_empty() {
                continue;
            }
            let candidate = PathBuf::from(base).join(rel);
            if candidate.exists() {
                return Some(BrowserInfo { name: name.to_string(), path: candidate, is_snap: false });
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
pub fn detect_browser() -> Option<BrowserInfo> {
    // Chrome first, then Chromium
    let candidates = [
        ("chrome", &["google-chrome", "google-chrome-stable"][..]),
        ("chromium", &["chromium-browser", "chromium"][..]),
    ];
    for (name, bins) in &candidates {
        for bin in *bins {
            if let Ok(path) = which::which(bin) {
                let is_snap = path.starts_with("/snap/");
                return Some(BrowserInfo { name: name.to_string(), path, is_snap });
            }
        }
    }
    None
}
