use std::env;

use crate::utils::check_heart_beat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ViteMode {
    Development,
    Manifest,
}

impl ViteMode {
    pub(crate) async fn discover<'a>(use_hb: bool, use_dev_server: bool, host: &'a str) -> ViteMode {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ViteConfig<'a> {
    /// The `path/to/manifest.json` file.
    /// Currently, Vite won't resolve relative paths, so please consider
    /// your current working directory as the root of it and start the path
    /// with a root node directory directly.
    /// 
    /// **Optionally and experimentally**, you can use the [`resolve_path`]
    /// method for the manifest file resolution. However, this method might
    /// come to fail at some point, and will also panic in the many situations
    /// described on its documentation.
    /// 
    /// # Example
    /// ```plaintext
    /// your_project/
    /// |-- public/
    /// |   |-- dist/
    /// |   |   |-- manifest.json
    /// |-- src/
    /// |   |-- main.rs // <-- you're here!
    /// ```
    /// 
    /// ```ignore
    /// 
    /// use vite_rust::{ViteConfig, utils::resolve_path};
    /// 
    /// let config = ViteConfig {
    ///     manifest_path: "public/dist/manifest.json",
    ///     // or
    ///     manifest_path: resolve_path(file!(), "../public/dist/manifest.json"),
    ///     // ...
    /// };
    /// ```
    pub manifest_path: &'a str,
    /// Defines which entrypoints Vite will use to generate the html `script`,
    /// `link` and `stylesheet` tags.
    /// 
    /// If `None` is provided, Vite will scan the manifest for files with
    /// `isEntry: true` property and consider them the entrypoints.
    pub entrypoints: Option<Vec<&'a str>>,
    /// If `None` is provided, Vite will discover which one to use considering:
    /// -   any of `RUST_ENV`, `NODE_ENV` or `APP_ENV` environment variables exists
    ///     and is set to `true`;
    /// -   Dev-server is running;
    /// -   Heart beat check is enabled.
    /// 
    /// By setting this option, the discovering phase will be skipped.
    /// Refer to the crate's `README.md` file to understand the way it decides which mode to pick.
    pub force_mode: Option<ViteMode>,
    /// Whether Vite should ping your vite dev-server to check if its running.
    /// If false, `ViteMode` will be set to `Development` if not forced by the configuration.
    pub use_heart_beat_check: bool,
    /// Whether dev server should be considered or not.
    /// 
    /// If false, `force_mode` should be either `Manifest` or `None`,
    /// otherwise, undefined behavior might occur.
    pub enable_dev_server: bool,
    /// The host in which your vite dev-server is running.
    /// Normally, it would be `"http://localhost:5173"`.
    /// 
    /// Please, do not forget the protocol (http, https)!
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
    /// let with_defaults_config = ViteConfig::new_with_defaults("path/to/manifest.json");
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
