fn main() {
    env_logger::init();

    let vite_resolved = vite_rust::resolve_manifest(
        "example/dist/.vite/manifest.json",
        vec!["src/www/main.tsx", "src/www/index.css"]
    );

    println!("{}", vite_resolved.unwrap());
}
