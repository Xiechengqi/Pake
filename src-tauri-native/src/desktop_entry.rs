use std::fs;

use crate::browser_detect::BrowserInfo;
use crate::config::NativeBrowserConfig;

/// Reproduce Chrome's WM_CLASS for `--app=<url>` on Linux.
///
/// Chrome builds the application name via (see Chromium source
/// `web_app_helpers.cc`):
///   GenerateApplicationNameFromURL(url) = host + "_" + path
///
/// Then `GetWMClassFromAppName` replaces illegal path characters
/// (including '/') with '_' and trims leading/trailing underscores.
///
/// Finally the WM_CLASS is:  "chrome-" + app_name + "-Default"
///
/// Examples:
///   https://gmail.com       -> chrome-gmail.com__-Default
///   https://godaddy.com/en  -> chrome-godaddy.com__en-Default
fn chrome_app_id(url: &str) -> String {
    let stripped = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    // Split into host and path (mirroring GURL::host() + GURL::path())
    let (host, path) = match stripped.find('/') {
        Some(i) => (&stripped[..i], &stripped[i..]),
        None => (stripped, "/"),
    };

    // Chrome: base::StrCat({url.host(), "_", url.path()})
    let app_name = format!("{}_{}", host, path);

    // Chrome: ReplaceIllegalCharactersInPath replaces '/' (and other
    // illegal chars) with '_', then trims leading/trailing '_'.
    let sanitized: String = app_name
        .chars()
        .map(|c| if c == '/' { '_' } else { c })
        .collect();
    let trimmed = sanitized.trim_matches('_');

    format!("chrome-{}-Default", trimmed)
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
