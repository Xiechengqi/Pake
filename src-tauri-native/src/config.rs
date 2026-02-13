use serde::Deserialize;

#[derive(Deserialize)]
pub struct NativeBrowserConfig {
    pub url: String,
    pub app_name: String,
    pub width: u32,
    pub height: u32,
}

pub fn load_config() -> NativeBrowserConfig {
    let raw = include_str!("../.pake/native_config.json");
    serde_json::from_str(raw).expect("Failed to parse native_config.json")
}
