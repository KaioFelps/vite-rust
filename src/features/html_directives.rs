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
            Regex::new(r"([ \t]*)@vite([ \t]*)").unwrap()
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
