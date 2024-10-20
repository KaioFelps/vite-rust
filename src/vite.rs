use std::sync::Arc;

use crate::error::ViteError;
use crate::manifest::Manifest;
use crate::utils::map_to_arc_vec;
use crate::config::{ViteConfig, ViteMode};

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
    ///     let mut vite_config = ViteConfig::new_with_defaults("example/dist/.vite/manifest.json");
    ///     vite_config.entrypoints = Some(vec!["src/www/main.tsx", "src/www/index.css"]);
    ///     vite_config.force_mode = Some(vite_rust::ViteMode::Manifest);
    ///     
    ///     let vite = Vite::new(vite_config).await.unwrap();
    /// 
    ///     let expected =
    ///         r#"<link rel="stylesheet" href="assets/index-BPvgi06w.css" />
    ///         <link rel="stylesheet" href="assets/main-Bx9V9zN2.css" />
    ///         <script type="module" src="assets/main-C4QS14El.js"></script>
    ///         <link rel="modulepreload" href="assets/react-CHdo91hT.svg" />"#;
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

    pub fn get_tags(&self) -> String {
        self.manifest.generate_html_tags(&self.entrypoints)
    }

    pub fn get_hmr_script(&self) -> String {
        return format!(
            r#"<script type="module" src="{}/@vite/client"></script>"#,
            &self.dev_server_host
        );
    }

    pub fn get_react_script(&self) -> String {
        return format!(r#"<script type="module">
            import RefreshRuntime from '{}/@react-refresh'
            RefreshRuntime.injectIntoGlobalHook(window)
            window.$RefreshReg$ = () => {{}}
            window.$RefreshSig$ = () => (type) => type
            window.__vite_plugin_react_preamble_installed__ = true
        </script>
        "#, &self.dev_server_host);
    }

    #[inline]
    pub fn get_hash(&self) -> &str { self.manifest.get_hash() }
}
