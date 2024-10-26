use std::sync::OnceLock;
use regex::Regex;

use crate::{Vite, ViteMode};

static VITE_DIRECTIVE: OnceLock<Regex> = OnceLock::new();
static VITE_REACT_DIRECTIVE: OnceLock<Regex> = OnceLock::new();
static VITE_ASSETS_DIRECTIVE: OnceLock<Regex> = OnceLock::new();
static VITE_HMR_DIRECTIVE: OnceLock<Regex> = OnceLock::new();

pub trait ViteDefaultDirectives {
    fn vite_directive(&self, html: &mut String);
    fn hmr_directive(&self, html: &mut String);
    fn assets_url_directive(&self, html: &mut String);
    fn react_directive(&self, html: &mut String);
}

impl ViteDefaultDirectives for Vite {
    /// Expands `@vite` directives in the given html template.
    /// 
    /// If [`ViteMode`] is "Development", script tags will be generated to the
    /// entry points and HMR client script will be also injected.
    /// 
    /// Otherwise, modulepreload, preload, stylesheet and script tags will be generated
    /// for the entry points and referenced assets.
    /// 
    /// # Arguments
    /// * `html`    - A mutable reference to a html string.
    /// 
    /// [`ViteMode`]: crate::ViteMode
    fn vite_directive(&self, html: &mut String) {
        let regex = VITE_DIRECTIVE.get_or_init(|| {
            Regex::new(r"([ \t]*)@vite([ \t]*\n?)$").unwrap()
        });

        let tags_or_scripts: String = self.get_resolved_vite_scripts();

        *html = regex.replace_all(html, |caps: &regex::Captures| {
            return format!("{}{}{}", &caps[1], tags_or_scripts, &caps[2]);
        }).to_string();
    }

    /// Replaces `@vite::asset("path/to/asset.file")` directives by the chunk's bundled file
    /// path. If there is no such an asset at the manifest, it will be replaced by an empty
    /// string.
    /// 
    /// If mode is [`ViteMode::Development`], it will be replaced by an URL referencing the
    /// vite dev-server.
    /// 
    /// `@vite::assets` is also valid.
    /// 
    /// # Arguments
    /// * `html`    - A mutable reference to a html string.
    fn assets_url_directive(&self, html: &mut String) {
        let regex = VITE_ASSETS_DIRECTIVE.get_or_init(|| {
            Regex::new(r#"([ \t]*)@vite::asset[s]?\(['"]?(.*)['"]?\)([ \t]*)"#).unwrap()
        });

        *html = regex.replace_all(html, |caps: &regex::Captures| {
            return format!(
                "{}{}{}",
                &caps[1],
                self.get_asset_url(&caps[2]),
                &caps[3]
            );
        }).to_string();
    }

    /// Expands a `@vite::hmr` directive by the HMR client script tag.
    /// If mode is [`ViteMode::Manifest`], it will be replaced by an empty
    /// string.
    /// 
    /// # Arguments
    /// * `html`    - A mutable reference to a html string.
    /// 
    /// [`ViteMode::Manifest`]: crate::ViteMode::Manifest
    fn hmr_directive(&self, html: &mut String) {
        let regex = VITE_HMR_DIRECTIVE.get_or_init(|| {
            Regex::new(r"([ \t]*)@vite::hmr([ \t]*)").unwrap()
        });

        *html = regex.replace_all(html, |caps: &regex::Captures| {
            return match self.mode {
                ViteMode::Manifest => "".into(),
                ViteMode::Development => format!("{}{}{}", &caps[1], self.get_hmr_script(), &caps[2])
            };
        }).to_string();
    }

    /// Expands a `@vite::react` directive to the [react fast refresh] script during [`ViteMode::Development`].
    /// Otherwise, it's replaced by an empty string.
    /// 
    /// # Arguments
    /// * `html`    - A mutable reference to a html string.
    /// 
    /// [`ViteMode::Development`]: crate::ViteMode::Development
    /// [react fast refresh]: https://vite.dev/guide/backend-integration
    fn react_directive(&self, html: &mut String) {
        let regex = VITE_REACT_DIRECTIVE.get_or_init(|| {
            Regex::new(r"([ \t]*)@vite::react([ \t]*)").unwrap()
        });

        *html = regex.replace_all(html, |caps: &regex::Captures| {
            return match self.mode {
                ViteMode::Development => format!("{}{}{}", &caps[1], self.get_react_script(), &caps[2]),
                ViteMode::Manifest => "".into()
            };
        }).to_string();
    }
}

#[cfg(test)]
mod test {
    use crate::features::html_directives::ViteDefaultDirectives;
    use crate::test_utils::NormalizeHtmlStrings;
    use crate::{Vite, ViteConfig, ViteMode};

    async fn get_vites() -> (Vite, Vite) {
        let mut conf = ViteConfig::new_with_defaults("tests/test-manifest.json");
        conf.entrypoints = Some(vec!["views/foo.js"]);
        conf.force_mode = Some(ViteMode::Development);
        let dev_vite = Vite::new(conf.clone()).await.unwrap();
        
        conf.force_mode = Some(ViteMode::Manifest);
        let manifest_vite = Vite::new(conf).await.unwrap();

        return (dev_vite, manifest_vite);
    }

    #[tokio::test]
    async fn test_vite_directive() {
        let (dev, manifest) = get_vites().await;

        let dev_expected = r#"
        @vite::react
        <script type="module" src="http://localhost:5173/views/foo.js"></script>
        <script type="module" src="http://localhost:5173/@vite/client"></script>
        "#;

        let manifest_expected = r#"
        @vite::react
        <link rel="stylesheet" href="assets/foo-5UjPuW-k.css" />
        <link rel="stylesheet" href="assets/shared-ChJ_j-JJ.css" />
        <script type="module" src="assets/foo-BRBmoGS9.js"></script>
        <link rel="modulepreload" href="assets/shared-B7PI925R.js" />
        "#;

        let mut dev_directive = "@vite::react\n@vite".to_string();
        let mut manifest_directive = dev_directive.clone();
        
        dev.vite_directive(&mut dev_directive);
        assert_eq!(dev_directive, dev_expected.__normalize_html_strings());
        
        manifest.vite_directive(&mut manifest_directive);
        assert_eq!(manifest_directive, manifest_expected.__normalize_html_strings());
    }

    #[tokio::test]
    async fn test_hmr_directive() {
        let (dev, manifest) = get_vites().await;

        let dev_expected = r#"<script type="module" src="http://localhost:5173/@vite/client"></script>"#;
        let manifest_expected = "";

        let mut dev_directive = "@vite::hmr".to_string();
        let mut manifest_directive = dev_directive.clone();
        
        dev.hmr_directive(&mut dev_directive);
        assert_eq!(dev_directive, dev_expected);
        
        manifest.hmr_directive(&mut manifest_directive);
        assert_eq!(manifest_directive, manifest_expected);
    }

    #[tokio::test]
    async fn test_react_directive() {
        let (dev, manifest) = get_vites().await;

        let dev_expected = r#"
            <script type="module">
                import RefreshRuntime from 'http://localhost:5173/@react-refresh'
                RefreshRuntime.injectIntoGlobalHook(window)
                window.$RefreshReg$ = () => {}
                window.$RefreshSig$ = () => (type) => type
                window.__vite_plugin_react_preamble_installed__ = true
            </script>
            "#;

        let manifest_expected = "";

        let mut dev_directive = "@vite::react".to_string();
        let mut manifest_directive = dev_directive.clone();
        
        dev.react_directive(&mut dev_directive);
        assert_eq!(dev_directive.__normalize_html_strings(), dev_expected.__normalize_html_strings());
        
        manifest.react_directive(&mut manifest_directive);
        assert_eq!(manifest_directive, manifest_expected);
    }

    #[tokio::test]
    async fn test_assets_directive() {
        let (dev, manifest) = get_vites().await;

        let mut dev_directive = "@vite::asset('baz.js')".to_string();
        let mut manifest_directive = dev_directive.clone();
        
        dev.assets_url_directive(&mut dev_directive);
        assert_eq!(dev_directive, "http://localhost:5173/baz.js");
        
        manifest.assets_url_directive(&mut manifest_directive);
        assert_eq!(manifest_directive, "assets/baz-B2H3sXNv.js");
    }
}
