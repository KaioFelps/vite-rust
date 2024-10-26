#![cfg(feature = "basic_directives")]

use vite_rust::features::html_directives::ViteDefaultDirectives;
use vite_rust::{Vite, ViteConfig, ViteMode};

fn normalize_html_strings<T>(input: T) -> String where T : ToString {
    input
    .to_string()
    .replace("\t", "     ")
    .lines()
    .map(str::trim)
    .collect::<Vec::<&str>>()
    .join("\n")
    .trim().to_string()
}

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
    <script type="module" src="http://localhost:5173/views/foo.js"></script>
    <script type="module" src="http://localhost:5173/@vite/client"></script>
    "#;

    let manifest_expected = r#"
    <link rel="stylesheet" href="assets/foo-5UjPuW-k.css" />
    <link rel="stylesheet" href="assets/shared-ChJ_j-JJ.css" />
    <script type="module" src="assets/foo-BRBmoGS9.js"></script>
    <link rel="modulepreload" href="assets/shared-B7PI925R.js" />
    "#;

    let mut dev_directive = "@vite".to_string();
    let mut manifest_directive = dev_directive.clone();
    
    dev.vite_directive(&mut dev_directive);
    assert_eq!(dev_directive, normalize_html_strings(dev_expected));
    
    manifest.vite_directive(&mut manifest_directive);
    assert_eq!(manifest_directive, normalize_html_strings(manifest_expected));
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
    assert_eq!(normalize_html_strings(dev_directive), normalize_html_strings(dev_expected));
    
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