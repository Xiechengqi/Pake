use std::fs;
use std::path::{Path, PathBuf};

pub fn ensure_extension(data_dir: &Path) -> PathBuf {
    let ext_dir = data_dir.join("pake-ext");
    fs::create_dir_all(&ext_dir).expect("Failed to create extension directory");
    fs::write(
        ext_dir.join("manifest.json"),
        include_str!("../extension/manifest.json"),
    )
    .unwrap_or_else(|e| eprintln!("Warning: failed to write extension manifest.json: {}", e));
    fs::write(ext_dir.join("nav.js"), include_str!("../extension/nav.js"))
        .unwrap_or_else(|e| eprintln!("Warning: failed to write extension nav.js: {}", e));
    fs::write(
        ext_dir.join("nav.css"),
        include_str!("../extension/nav.css"),
    )
    .unwrap_or_else(|e| eprintln!("Warning: failed to write extension nav.css: {}", e));
    ext_dir
}
