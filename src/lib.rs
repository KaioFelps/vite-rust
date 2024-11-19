mod asset;
mod chunk;
mod config;
pub mod error;
mod manifest;
mod vite;

#[cfg(test)]
mod test_utils;

const CLIENT_SCRIPT_PATH: &str = r#"@vite/client"#;

pub mod features;
pub mod utils;

pub use config::ViteConfig;
pub use config::ViteMode;
pub use error::ViteError;
pub use vite::Vite;
