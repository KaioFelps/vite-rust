use vite_rust::ViteConfig;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut vite_config = ViteConfig::new_with_defaults("example/dist/.vite/manifest.json");
    vite_config.entrypoints = Some(vec!["src/www/main.tsx", "src/www/index.css"]);
    vite_config.force_mode = Some(vite_rust::ViteMode::Manifest);

    let vite = vite_rust::Vite::new(vite_config).await;

    if vite.is_err() {
        panic!("{:#?}", vite.unwrap_err());
    }
    
    let vite = vite.unwrap();

    let vite_resolved = vite.get_tags();

    println!("{}", vite_resolved);
    println!("{}", vite.get_hash());
}
