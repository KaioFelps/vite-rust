#[cfg(test)]
mod test {
    #[tokio::test]
    async fn getting_started() {
        use vite_rust::ViteConfig;
        let vite_config: vite_rust::ViteConfig = vite_rust::ViteConfig::default()
            .set_manifest_path("tests/test-manifest.json")
            .set_entrypoints(vec!["views/bar.js", "views/foo.js"]);

        let vite = vite_rust::Vite::new(vite_config.clone()).await.unwrap();

        let tags = vite.get_tags(); // get html tags from Manifest and given entrypoints
        let hmr_script = vite.get_hmr_script(); // get hmr script if Mode is set to Develpment. Empty string otherwise

        assert_eq!(
            ViteConfig {
                entrypoints: Some(vec!["views/bar.js", "views/foo.js"]),
                manifest_path: Some("tests/test-manifest.json"),
                ..Default::default()
            },
            vite_config
        );
        assert!(tags.is_ok());
        assert!(hmr_script.is_empty()); // no hmr script in ViteMode::Manifest
    }

    #[tokio::test]
    async fn directives() {
        use vite_rust::{features::html_directives::ViteDefaultDirectives, Vite, ViteConfig};

        let vite = Vite::new(
            ViteConfig::default()
                .set_manifest_path("tests/test-manifest.json")
                .set_entrypoints(vec!["views/bar.js", "views/foo.js"])
                .set_heart_beat_retries_limit(1)
                .set_force_mode(vite_rust::ViteMode::Development),
        )
        .await
        .unwrap();

        let mut template = r#"
        <htmL>
            <head>
                @vite::react
                @vite
            </head>
            <body>
                <div id="root"></div>
            </body>
        </html>
        "#
        .to_string();

        vite.react_directive(&mut template);
        if let Err(err) = vite.vite_directive(&mut template) {
            log::error!("Failed to resolve vite directive: {}", err);
        };

        assert_eq!(
            r#"
        <htmL>
            <head>
                <script type="module">
                import RefreshRuntime from 'http://localhost:5173/@react-refresh'
                RefreshRuntime.injectIntoGlobalHook(window)
                window.$RefreshReg$ = () => {}
                window.$RefreshSig$ = () => (type) => type
                window.__vite_plugin_react_preamble_installed__ = true
            </script>
                <script type="module" src="http://localhost:5173/views/bar.js"></script>
<script type="module" src="http://localhost:5173/views/foo.js"></script>
<script type="module" src="http://localhost:5173/@vite/client"></script>
            </head>
            <body>
                <div id="root"></div>
            </body>
        </html>
        "#,
            template
        );
    }
}
