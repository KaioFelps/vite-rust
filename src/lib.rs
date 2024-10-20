pub mod error;
mod asset;
mod chunk;
mod manifest;
mod vite;
mod utils;
mod config;

const CLIENT_SCRIPT_PATH: &'static str = r#"@vite/client"#;

pub use error::ViteError;
pub use vite::Vite;
pub use config::ViteConfig;
pub use config::ViteMode;