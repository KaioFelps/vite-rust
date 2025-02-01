use std::env;

use crate::utils::check_heart_beat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ViteMode {
    Development,
    Manifest,
}

impl ViteMode {
    pub(crate) async fn discover(
        use_hb: bool,
        use_dev_server: bool,
        host: &str,
        hb_retries: u8,
    ) -> ViteMode {
        if !use_hb {
            return ViteMode::Development;
        }

        if !use_dev_server {
            return ViteMode::Manifest;
        }

        if is_production() {
            return ViteMode::Manifest;
        }

        let dev_server_is_ok = check_heart_beat(host, None, hb_retries).await;

        match dev_server_is_ok {
            true => ViteMode::Development,
            false => ViteMode::Manifest,
        }
    }
}

fn is_production() -> bool {
    env::vars().any(|(k, v)| {
        if [
            "RUST_ENV",
            "NODE_ENV",
            "APP_ENV",
            "__TEST_APP_ENV",
            "LOCO_ENV",
            "RAILS_ENV",
        ]
        .contains(&k.as_str())
        {
            return v == "production";
        }

        false
    })
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
    ///     manifest_path: Some("public/dist/manifest.json"),
    ///     // or
    ///     manifest_path: Some(resolve_path(file!(), "../public/dist/manifest.json")),
    ///     // ...
    /// };
    /// ```
    pub manifest_path: Option<&'a str>,
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
    /// How many times heartbeat checker should try before fallbacking.
    pub heart_beat_retries_limit: Option<u8>,
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
    /// Prefix assets path with the given `str`.
    pub prefix: Option<&'a str>,
    /// Add a custom domain to prefix every asset URL with.
    pub app_url: Option<&'a str>,
}

impl<'a> ViteConfig<'a> {
    /// Creates a new `ViteConfig` instance with `manifest_path` and `entrypoints` fields set.
    pub fn new(manifest_path: &'a str, entrypoints: Vec<&'a str>) -> Self {
        ViteConfig::default()
            .set_manifest_path(manifest_path)
            .set_entrypoints(entrypoints)
    }

    pub fn set_manifest_path(mut self, manifest_path: &'a str) -> Self {
        self.manifest_path = Some(manifest_path);
        self
    }

    pub fn set_entrypoints(mut self, entrypoints: Vec<&'a str>) -> Self {
        self.entrypoints = Some(entrypoints);
        self
    }

    pub fn set_force_mode(mut self, mode: ViteMode) -> Self {
        self.force_mode = Some(mode);
        self
    }
    pub fn set_server_host(mut self, server_host: &'a str) -> Self {
        self.server_host = Some(server_host);
        self
    }

    pub fn set_heart_beat_retries_limit(mut self, limit: u8) -> Self {
        self.heart_beat_retries_limit = Some(limit);
        self
    }

    pub fn without_heart_beat_check(mut self) -> Self {
        self.use_heart_beat_check = false;
        self
    }

    pub fn without_dev_server(mut self) -> Self {
        self.enable_dev_server = false;
        self
    }

    pub fn set_prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn set_app_url(mut self, app_url: &'a str) -> Self {
        self.app_url = Some(app_url);
        self
    }
}

impl Default for ViteConfig<'_> {
    /// Create a `ViteConfig` instance.
    ///
    /// You can create your config by directly instantiating the struct, or
    /// by using some default options. Note that you **must set the manifest
    /// path**.
    ///
    /// # Example
    /// ```rust
    /// use vite_rust::ViteConfig;
    ///
    /// let manual_config = ViteConfig {
    ///     manifest_path: Some("path/to/manifest.json"),
    ///     entrypoints: None, // Vite can discover them by itself
    ///     force_mode: None, // Vite can discover it too
    ///     use_heart_beat_check: true,
    ///     enable_dev_server: true,
    ///     server_host: Some("http://localhost:5173"),
    ///     heart_beat_retries_limit: Some(5),
    ///     prefix: None,
    ///     app_url: None,
    /// };
    ///
    /// let with_defaults_config = ViteConfig::default().set_manifest_path("path/to/manifest.json");
    ///
    /// assert_eq!(manual_config, with_defaults_config);
    /// ```
    fn default() -> Self {
        Self {
            enable_dev_server: true,
            entrypoints: None,
            manifest_path: None,
            force_mode: None,
            server_host: Some("http://localhost:5173"),
            use_heart_beat_check: true,
            heart_beat_retries_limit: Some(5),
            prefix: None,
            app_url: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::{config::is_production, ViteMode};

    #[tokio::test]
    async fn test_discover() {
        let host = "http://localhost:3000";
        let hb_retries = 1;

        assert_eq!(
            ViteMode::Development,
            ViteMode::discover(false, true, host, hb_retries).await
        );

        assert_eq!(
            ViteMode::Manifest,
            ViteMode::discover(true, false, host, hb_retries).await
        );
    }

    #[test]
    fn test_is_production() {
        env::set_var("__TEST_APP_ENV", "production");
        assert!(is_production());

        env::remove_var("__TEST_APP_ENV");
        assert!(!is_production());
    }
}
