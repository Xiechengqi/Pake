use std::fs;

use crate::browser_detect::BrowserInfo;
use crate::config::NativeBrowserConfig;

/// Chrome --app mode generates WM_CLASS / Wayland app_id as:
///   chrome-{sanitized_url}-Default
/// where sanitized_url replaces '/' with '_' and strips the scheme.
fn chrome_app_id(url: &str) -> String {
    let stripped = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    // Chrome normalizes the URL (appends trailing /) then replaces each '/' with '__'
    let with_slash = if stripped.ends_with('/') {
        stripped.to_string()
    } else {
        format!("{}/", stripped)
    };
    let sanitized = with_slash.replace('/', "__");
    format!("chrome-{}-Default", sanitized)
}

/// Install a .desktop file so the taskbar shows the app name instead of
/// Chrome's auto-generated WM_CLASS like "chrome-youtube.com__-Default".
pub fn install(config: &NativeBrowserConfig, _browser: &BrowserInfo) {
    let Some(data_home) = dirs::data_dir() else {
        return;
    };
    let apps_dir = data_home.join("applications");
    fs::create_dir_all(&apps_dir).ok();

    let wm_class = chrome_app_id(&config.url);
    let desktop_id = format!("pake-{}.desktop", config.app_name.to_lowercase());
    let content = format!(
        "[Desktop Entry]\n\
         Name={name}\n\
         Type=Application\n\
         StartupWMClass={wm_class}\n\
         NoDisplay=true\n",
        name = config.app_name,
        wm_class = wm_class,
    );

    fs::write(apps_dir.join(&desktop_id), content).ok();
}
