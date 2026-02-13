use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub fn detect_chrome() -> Option<PathBuf> {
    let candidates = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    ];
    for candidate in &candidates {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return Some(path);
        }
    }
    // Also check ~/Applications
    if let Some(home) = dirs::home_dir() {
        let user_chrome = home
            .join("Applications")
            .join("Google Chrome.app")
            .join("Contents")
            .join("MacOS")
            .join("Google Chrome");
        if user_chrome.exists() {
            return Some(user_chrome);
        }
    }
    None
}

#[cfg(target_os = "windows")]
pub fn detect_chrome() -> Option<PathBuf> {
    // Try registry first
    if let Ok(hklm) = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
        .open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe")
    {
        if let Ok(path) = hklm.get_value::<String, _>("") {
            let p = PathBuf::from(&path);
            if p.exists() {
                return Some(p);
            }
        }
    }

    // Fallback to common paths
    let program_files = [
        std::env::var("ProgramFiles").unwrap_or_default(),
        std::env::var("ProgramFiles(x86)").unwrap_or_default(),
        std::env::var("LocalAppData").unwrap_or_default(),
    ];
    for base in &program_files {
        if base.is_empty() {
            continue;
        }
        let candidate = PathBuf::from(base)
            .join("Google")
            .join("Chrome")
            .join("Application")
            .join("chrome.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(target_os = "linux")]
pub fn detect_chrome() -> Option<PathBuf> {
    let names = ["google-chrome", "google-chrome-stable"];
    for name in &names {
        if let Ok(path) = which::which(name) {
            return Some(path);
        }
    }
    None
}
