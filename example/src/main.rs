use vite_rust::{utils::resolve_path, ViteConfig};

#[tokio::main]
async fn main() {
    env_logger::init();

    let manifest_path = resolve_path(file!(), "../dist/.vite/manifest.json");
    
    let mut vite_config = ViteConfig::new_with_defaults(&manifest_path);
    vite_config.entrypoints = Some(vec!["src/www/main.tsx", "src/www/index.css"]);

    let vite = vite_rust::Vite::new(vite_config).await;

    if vite.is_err() {
        panic!("{:#?}", vite.unwrap_err());
    }
    
    let vite = vite.unwrap();

    let vite_resolved = vite.get_tags();

    println!("{}", vite_resolved);
    println!("{}", vite.get_hash());
}
