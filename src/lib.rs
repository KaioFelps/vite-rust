pub mod error;
mod asset;
mod chunk;
mod manifest;

use error::ViteError;
use manifest::Manifest;

pub fn resolve_manifest(manifest_path: &str, entries: Vec<&str>) -> Result<String, ViteError> {
    let manifest = Manifest::new(manifest_path)?;
    let html = manifest.generate_html_tags(entries);
    return Ok(html);
}