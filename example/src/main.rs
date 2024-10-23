use std::{fs::File, io::Read, sync::Arc};
use tower_http::services::ServeDir;
use vite_rust::{features::html_directives::ViteDefaultDirectives, utils::resolve_path, Vite, ViteConfig};
use axum::extract::State;
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use axum_macros::debug_handler;

struct AppState {
    vite: Vite
}

#[debug_handler]
async fn home(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut template = String::new();
    let _ = File::open(resolve_path(file!(), "../index.html"))
        .unwrap()
        .read_to_string(&mut template);

    state.vite.vite_directive(&mut template);
    state.vite.react_directive(&mut template);
    state.vite.assets_url_directive(&mut template);

    Html::from(template)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let manifest_path: String = resolve_path(file!(), "../dist/.vite/manifest.json");
    let mut vite_config: ViteConfig<'_> = ViteConfig::new_with_defaults(&manifest_path);
    vite_config.entrypoints = Some(vec!["src/www/main.tsx", "src/www/index.css"]);

    let vite = vite_rust::Vite::new(vite_config).await.unwrap();

    let state: Arc<AppState> = Arc::new(AppState {
        vite
    });

    let fallback_404_service = handle_404.into_service();
    let dist_dir = ServeDir::new("dist/assets").not_found_service(fallback_404_service.clone());
    let public_dir = ServeDir::new("public").not_found_service(fallback_404_service);

    let app = Router::new()
        .route("/", get(home))
        .nest_service("/assets", dist_dir)
        .fallback_service(public_dir)
        .with_state(Arc::clone(&state));

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Listening on address http://127.0.0.1:3000");
    axum::serve(listener, app).await
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}