use std::env;

use crate::asset::Asset;
use crate::config::{ViteConfig, ViteMode};
use crate::error::{ViteError, ViteErrorKind};
use crate::manifest::Manifest;
use crate::CLIENT_SCRIPT_PATH;

pub(crate) type Entrypoints = Vec<Box<str>>;

#[derive(Debug)]
pub struct Vite {
    pub(crate) manifest: Option<Manifest>,
    pub(crate) entrypoints: Entrypoints,
    pub(crate) mode: ViteMode,
    pub(crate) dev_server_host: &'static str,
    pub(crate) prefix: Option<&'static str>,
    pub(crate) app_url: &'static str,
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
    ///     let mut vite_config = ViteConfig::default()
    ///         .set_manifest_path("tests/test-manifest.json")
    ///         .set_entrypoints(vec!["views/foo.js"])
    ///         .set_force_mode(vite_rust::ViteMode::Manifest);
    ///     
    ///     let vite = Vite::new(vite_config).await.unwrap();
    ///
    ///     let expected =
    ///         r#"<link rel="stylesheet" href="/assets/foo-5UjPuW-k.css" />
    ///         <link rel="stylesheet" href="/assets/shared-ChJ_j-JJ.css" />
    ///         <script type="module" src="/assets/foo-BRBmoGS9.js"></script>
    ///         <link rel="modulepreload" href="/assets/shared-B7PI925R.js" />"#;
    ///
    ///     let expected = expected.replace("\t", "     ")
    ///         .lines()
    ///         .map(str::trim)
    ///         .collect::<Vec::<&str>>()
    ///         .join("\n");
    ///     
    ///     assert_eq!(vite.get_tags().unwrap(), expected);
    /// }
    /// ```
    ///
    /// [`ViteConfig`]: crate::config::ViteConfig
    pub async fn new(config: ViteConfig<'_>) -> Result<Vite, ViteError> {
        let dev_host = Box::leak(
            config
                .server_host
                .unwrap_or("http://localhost:5173")
                .to_string()
                .into_boxed_str(),
        );

        let mode = match config.force_mode {
            Some(mode) => mode,
            None => {
                ViteMode::discover(
                    config.use_heart_beat_check,
                    config.enable_dev_server,
                    dev_host,
                    config.heart_beat_retries_limit.unwrap(),
                )
                .await
            }
        };

        let manifest = if mode.eq(&ViteMode::Manifest) || config.entrypoints.is_none() {
            if let Some(manifest_path) = config.manifest_path {
                Some(Manifest::new(manifest_path)?)
            } else {
                panic!(
                    "Tried to start Vite in Manifest mode, but no manifest.json file has been set."
                );
            }
        } else {
            None
        };

        let entrypoints: Entrypoints = match config.entrypoints {
            Some(entrypoints) => entrypoints.into_iter().map(|entry| entry.into()).collect(),
            None => match &manifest {
                Some(manifest) => manifest
                    .get_manifest_entries()
                    .into_iter()
                    .map(|entry| entry.into())
                    .collect(),
                None => {
                    panic!("Tried to start Vite without entrypoints set nor manifest.json file");
                }
            },
        };

        let prefix = resolve_prefix(config.prefix);

        let app_url = resolve_app_url(config.app_url);

        Ok(Vite {
            entrypoints,
            manifest,
            mode,
            dev_server_host: dev_host,
            prefix,
            app_url,
        })
    }

    /// Generates assets HTML tags from `manifest.json` file.
    ///
    /// # Errors
    /// Returns a `ViteError` if there is no Manifest.
    ///
    /// # Panics
    /// Might panic if the target file doesn't exist.
    pub fn get_tags(&self) -> Result<String, ViteError> {
        match &self.manifest {
            Some(manifest) => {
                Ok(manifest.generate_html_tags(&self.entrypoints, self.prefix, self.app_url))
            }
            None => Err(ViteError::new(
                "Tried to get html tags from manifest, but there is no manifest file.",
                ViteErrorKind::Manifest,
            )),
        }
    }

    /// Generates scripts and stylesheet link HTML tags referencing
    /// the entrypoints directly from the Vite dev-server.
    pub fn get_development_scripts(&self) -> Result<String, ViteError> {
        let mut tags = vec![];

        for entry in self.entrypoints.iter() {
            if entry.ends_with(".css") {
                tags.push(Asset::StyleSheet(self.get_asset_url(entry)?).into_html());
            } else {
                tags.push(Asset::EntryPoint(self.get_asset_url(entry)?).into_html());
            }
        }

        Ok(tags.join("\n"))
    }

    /// Generates HTML tags considering the current [`ViteMode`]:
    /// -   If `Development` mode, calls `Vite::get_development_scripts()` and `Vite::get_hmr_script()`
    ///     and return a concatenation of their returns;
    /// -   If `Manifest` mode, calls `Vite::get_tags()` and return the assets HTML tags.
    ///
    /// # Errors
    /// Returns a `ViteError` instance if mode is `Manifest` and there is no Manifest.
    pub fn get_resolved_vite_scripts(&self) -> Result<String, ViteError> {
        match self.mode {
            ViteMode::Development => Ok(format!(
                "{}\n{}",
                self.get_development_scripts()?,
                self.get_hmr_script()
            )),
            ViteMode::Manifest => self.get_tags(),
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
                    &self.dev_server_host, CLIENT_SCRIPT_PATH
                )
            }
            ViteMode::Manifest => "".to_string(),
        }
    }

    /// Returns the bundled file by the given original file's path. If it is not present in the
    /// manifest file, an empty string is returned.
    ///
    /// # Arguments
    /// - `path`    - the root-relative path to an asset file. E.g. "src/assets/react.svg".
    pub fn get_asset_url(&self, path: &str) -> Result<String, ViteError> {
        let path = path.strip_prefix("/").unwrap_or(path).replace("'", "");

        match &self.mode {
            ViteMode::Development => Ok(format!("{}/{}", self.dev_server_host, path)),
            ViteMode::Manifest => match &self.manifest {
                Some(manifest) => Ok(manifest.get_asset_url(&path, self.prefix, self.app_url)),
                None => Err(ViteError::new(
                    "Tried to get asset's URL from manifest, but there is no manifest file.",
                    ViteErrorKind::Manifest,
                )),
            },
        }
    }

    /// Returns the [react fast refresh script] relative to the current Vite dev-server URL.
    ///
    /// [react fast refresh script]: https://vite.dev/guide/backend-integration
    pub fn get_react_script(&self) -> String {
        format!(
            r#"<script type="module">
                import RefreshRuntime from '{}/@react-refresh'
                RefreshRuntime.injectIntoGlobalHook(window)
                window.$RefreshReg$ = () => {{}}
                window.$RefreshSig$ = () => (type) => type
                window.__vite_plugin_react_preamble_installed__ = true
            </script>"#,
            &self.dev_server_host
        )
    }

    /// Returns the current `manifest.json` file hash. Might be used for
    /// assets versioning.
    ///
    /// The resultant string is a hex-encoded MD5 hash.
    #[inline]
    pub fn get_hash(&self) -> Option<&str> {
        match &self.manifest {
            Some(manifest) => Some(manifest.get_hash()),
            None => None,
        }
    }

    /// Returns the Vite instance's dev-server URL.
    pub fn get_dev_server_url(&self) -> &str {
        self.dev_server_host
    }

    /// Returns the current Vite instance's mode.
    pub fn mode(&self) -> &ViteMode {
        &self.mode
    }
}

pub(crate) fn resolve_prefix(prefix: Option<&str>) -> Option<&'static str> {
    if let Some(prefix) = prefix {
        if prefix.is_empty() || prefix.eq("/") {
            return None;
        }

        let prefix = prefix.strip_prefix("/").unwrap_or(prefix);
        let prefix = prefix.strip_suffix("/").unwrap_or(prefix);

        return Some(Box::leak(prefix.to_string().into_boxed_str()));
    }

    None
}

pub(crate) fn resolve_app_url(app_url: Option<&str>) -> &'static str {
    if let Some(app_url) = app_url {
        let app_url = app_url.strip_suffix("/").unwrap_or(app_url);

        return Box::leak(app_url.to_string().into_boxed_str());
    }

    let app_url = if let Ok(app_url) = env::var("APP_URL") {
        app_url.strip_suffix("/").unwrap_or(&app_url).to_string()
    } else {
        String::new()
    };

    Box::leak(app_url.into_boxed_str())
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::vite::{resolve_app_url, resolve_prefix};

    #[test]
    fn test_resolve_prefix() {
        const EXPECTED_RESULT: &str = "bundle";

        assert_eq!(EXPECTED_RESULT, resolve_prefix(Some("bundle")).unwrap());
        assert_eq!(EXPECTED_RESULT, resolve_prefix(Some("/bundle")).unwrap());
        assert_eq!(EXPECTED_RESULT, resolve_prefix(Some("bundle/")).unwrap());
        assert_eq!(EXPECTED_RESULT, resolve_prefix(Some("/bundle/")).unwrap());
    }
    #[test]
    fn test_resolve_app_url() {
        const EXPECTED_RESULT: &str = "http://foo.baz";

        assert_eq!(EXPECTED_RESULT, resolve_app_url(Some("http://foo.baz/")));
        assert_eq!(EXPECTED_RESULT, resolve_app_url(Some("http://foo.baz")));

        assert_eq!(resolve_app_url(None), "");

        env::set_var("APP_URL", "http://foo.baz");

        assert_eq!(resolve_app_url(None), EXPECTED_RESULT);

        env::remove_var("APP_URL");
    }
}
