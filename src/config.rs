use std::env;

use crate::utils::check_heart_beat;

#[derive(Debug, PartialEq, Eq)]
pub enum ViteMode {
    Development,
    Manifest,
}

impl ViteMode {
    pub async fn discover<'a>(use_hb: bool, use_dev_server: bool, host: &'a str) -> ViteMode {
        if !use_hb {
            return ViteMode::Development;
        }

        if !use_dev_server {
            return ViteMode::Manifest;
        }
        
        let is_production = env::vars().any(|(k, v)| {
            if ["RUST_ENV", "NODE_ENV", "APP_ENV"].contains(&k.as_str()) {
                return v.parse::<bool>().unwrap_or(false);
            }

            return false;
        });

        if is_production { return ViteMode::Manifest; }
        
        let dev_server_is_ok = check_heart_beat(host, None).await;

        match dev_server_is_ok {
            true => ViteMode::Development,
            false => ViteMode::Manifest,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ViteConfig<'a> {
    pub manifest_path: &'a str,
    // if entrypoints are not set, vite-rust will consider every
    // manifest's chunk that has `isEntry: true` as an entrypoint.
    pub entrypoints: Option<Vec<&'a str>>,
    // leave it empty for auto-discovering
    pub force_mode: Option<ViteMode>,
    pub use_heart_beat_check: bool,
    pub enable_dev_server: bool,
    pub server_host: Option<&'a str>,
}

impl<'a> ViteConfig<'a> {
    /// Create a `ViteConfig` instance.
    /// 
    /// You can create your config by directly instantiating the struct, or
    /// by using some default options and defining only the most critical fields:
    /// 
    /// # Example
    /// ```rust
    /// use vite_rust::ViteConfig;
    /// 
    /// let manual_config = ViteConfig {
    ///     manifest_path: "path/to/manifest.json",
    ///     entrypoints: None, // Vite can discover them by itself
    ///     force_mode: None, // Vite can discover it too
    ///     use_heart_beat_check: true,
    ///     enable_dev_server: true,
    ///     server_host: Some("http://localhost:5173")
    /// };
    /// 
    /// let with_defaults_config = ViteConfig::new_with_defaults("path/to/manifest.json", None);
    /// 
    /// assert_eq!(manual_config, with_defaults_config);
    /// ```
    pub fn new_with_defaults(manifest_path: &'a str) -> Self {
        ViteConfig {
            enable_dev_server: true,
            entrypoints: None,
            manifest_path,
            force_mode: None,
            server_host: Some("http://localhost:5173"),
            use_heart_beat_check: true,
        }
    }
}
