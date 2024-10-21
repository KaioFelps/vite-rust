use std::sync::Arc;

use crate::asset::Asset;
use crate::error::ViteError;
use crate::manifest::Manifest;
use crate::utils::map_to_arc_vec;
use crate::config::{ViteConfig, ViteMode};
use crate::CLIENT_SCRIPT_PATH;

#[derive(Debug)]
pub struct Vite {
    pub(crate) manifest: Manifest,
    pub(crate) entrypoints: Arc<Vec<String>>,
    pub(crate) mode: ViteMode,
    pub(crate) dev_server_host: String,
}

impl Vite {
    /// Creates a new Vite instance.
    /// 
    /// # Arguments
    /// * `config`  - a [`ViteConfig<'_>`] instance.
    /// 
    /// # Errors
    /// Returns `Err` if the given manifest path is not valid.
    /// 
    /// # Example
    /// ```rust
    /// use vite_rust::{Vite, ViteConfig};
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut vite_config = ViteConfig::new_with_defaults("tests/test-manifest.json");
    ///     vite_config.entrypoints = Some(vec!["views/foo.js"]);
    ///     vite_config.force_mode = Some(vite_rust::ViteMode::Manifest);
    ///     
    ///     let vite = Vite::new(vite_config).await.unwrap();
    /// 
    ///     let expected =
    ///         r#"<link rel="stylesheet" href="assets/foo-5UjPuW-k.css" />
    ///         <link rel="stylesheet" href="assets/shared-ChJ_j-JJ.css" />
    ///         <script type="module" src="assets/foo-BRBmoGS9.js"></script>
    ///         <link rel="modulepreload" href="assets/shared-B7PI925R.js" />"#;
    /// 
    ///     let expected = expected.replace("\t", "     ")
    ///         .lines()
    ///         .map(str::trim)
    ///         .collect::<Vec::<&str>>()
    ///         .join("\n");
    ///     
    ///     assert_eq!(vite.get_tags(), expected);
    /// }
    /// ```
    /// 
    /// [`ViteConfig`]: crate::config::ViteConfig
    pub async fn new<'b>(config: ViteConfig<'b>) -> Result<Vite, ViteError> {
        let manifest = Manifest::new(config.manifest_path)?;

        let entrypoints= match config.entrypoints {
            Some(entrypoints) => map_to_arc_vec(entrypoints),
            None => map_to_arc_vec(manifest.get_manifest_entries()),
        };

        let dev_host = config.server_host.unwrap_or("http://localhost:5173").to_string();

        let mode = match config.force_mode {
            Some(mode) => mode,
            None => ViteMode::discover(
                config.use_heart_beat_check,
                config.enable_dev_server,
                &dev_host
            ).await
        };

        return Ok(Vite {
            entrypoints,
            manifest,
            mode,
            dev_server_host: dev_host
        });
    }

    /// Generates assets HTML tags from `manifest.json` file.
    /// 
    /// # Panics
    /// Might panic if the file doesn't exist.
    pub fn get_tags(&self) -> String {
        self.manifest.generate_html_tags(&self.entrypoints)
    }

    /// Generates scripts and stylesheet link HTML tags referencing
    /// the entrypoints directly from the Vite dev-server.
    pub fn get_development_scripts(&self) -> String {
        let mut tags = vec![];

        for entry in self.entrypoints.iter() {
            if entry.ends_with(".css") {
                tags.push(Asset::StyleSheet(self.get_asset_url(entry)).to_html());
            } else {
                tags.push(Asset::EntryPoint(self.get_asset_url(entry)).to_html());
            }
        }

        return tags.join("\n");
    }

    /// Generates HTML tags considering the current [`ViteMode`]:
    /// -   If `Development` mode, calls `Vite::get_development_scripts()` and `Vite::get_hmr_script()`
    ///     and return a concatenation of their returns;
    /// -   If `Manifest` mode, calls `Vite::get_tags()` and return the assets HTML tags.
    pub fn get_resolved_vite_scripts(&self) -> String {
        match self.mode {
            ViteMode::Development => format!("{}\n{}", self.get_development_scripts(), self.get_hmr_script()),
            ViteMode::Manifest => self.get_tags()
        }
    }

    /// Returns a script tag referencing the Hot Module Reload client script from the Vite dev-server.
    /// 
    /// If [`ViteMode`] is set to `Manifest`, only an empty string is returned.
    pub fn get_hmr_script(&self) -> String {
        match self.mode {
            ViteMode::Development => {
                format!(
                    r#"<script type="module" src="{}/{}"></script>"#,
                    &self.dev_server_host,
                    CLIENT_SCRIPT_PATH
                )
            },
            ViteMode::Manifest => "".to_string()
        }
    }

    /// Returns the bundled file by the given original file's path. If it is not present in the
    /// manifest file, an empty string is returned.
    /// 
    /// # Arguments
    /// - `path`    - the root-relative path to an asset file. E.g. "src/assets/react.svg".
    pub fn get_asset_url(&self, path: &str) -> String {
        let path = if path.starts_with("/") { &path[1..] } else { path };
        let path = path.replace("'", "");

        match &self.mode {
            ViteMode::Development => format!("{}/{}", self.dev_server_host, path),
            ViteMode::Manifest => self.manifest.get_asset_url(&path).to_string(),
        }
    }

    /// Returns the [react fast refresh script] relative to the current Vite dev-server URL.
    /// 
    /// [react fast refresh script]: https://vite.dev/guide/backend-integration
    pub fn get_react_script(&self) -> String {
        return format!(
            r#"<script type="module">
                import RefreshRuntime from '{}/@react-refresh'
                RefreshRuntime.injectIntoGlobalHook(window)
                window.$RefreshReg$ = () => {{}}
                window.$RefreshSig$ = () => (type) => type
                window.__vite_plugin_react_preamble_installed__ = true
            </script>"#,
            &self.dev_server_host
        );
    }

    /// Returns the current `manifest.json` file hash. Might be used for
    /// assets versioning.
    /// 
    /// The resultant string is a hex-encoded MD5 hash.
    #[inline]
    pub fn get_hash(&self) -> &str { self.manifest.get_hash() }

    /// Returns the Vite instance's dev-server URL.
    pub fn get_dev_server_url(&self) -> &str { &self.dev_server_host }

    /// Returns the current Vite instance's mode.
    pub fn mode(&self) -> &ViteMode { &self.mode }
}
