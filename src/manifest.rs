use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use serde::Deserialize;

use crate::asset::Asset;
use crate::chunk::Chunk;
use crate::error::{ViteError, ViteErrorKind};

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub(crate) struct Manifest {
    manifest: HashMap<String, Chunk>,
}

impl Manifest {
    pub fn new(path: &str) -> Result<Self, ViteError> {
        let file = match File::open(path) {
            Err(err) => {
                return Err(ViteError::new(
                    format!("Failed to open manifest at {}: {}", path, err),
                    ViteErrorKind::Manifest
                ));
            },
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let manifest = serde_json::from_reader(reader);
        
        return match manifest {
            Err(err) => Err(ViteError::new(
                format!("Failed to parse manifest json: {}", err),
                ViteErrorKind::Manifest
            )),
            Ok(manifest) => Ok(manifest)
        };
    }

    pub fn generate_html_tags(&self, entrypoints: Vec<&str>) -> String {
        if self.manifest.is_empty() {
            log::warn!("Manifest is empty. Empty string being returned from `Manifest::generate_html_tags`.");
            return "".into();
        }

        let mut discovered_assets = HashSet::<Asset>::new();

        entrypoints
            .into_iter()
            .for_each(|entry| {
                let entry_chunk = &self.manifest[entry];
                let entry_as_asset = Asset::EntryPoint(entry_chunk.file.clone());

                if !discovered_assets.contains(&entry_as_asset) {
                    discovered_assets.insert(entry_as_asset);
                    self.iterate_over_chunk_assets(&mut discovered_assets, entry_chunk);
                }
            });

        let mut assets = discovered_assets.into_iter().collect::<Vec<Asset>>();
        // Puts the assets in the following order: stylesheets > entries > preloads
        assets.sort();

        return assets
            .into_iter()
            .map(|asset| asset.to_html())
            .collect::<Vec<String>>()
            .join("
            ");
    }

    fn iterate_over_chunk_assets(&self, set: &mut HashSet<Asset>, chunk: &Chunk) {
        for asset in chunk.assets_iter() {
            if !set.contains(&asset) {
                set.insert(asset);
            }
        }

        if chunk.is_entry {
            chunk.imports.iter().for_each(|import| {
                let import_chunk = &self.manifest[import];
                set.insert(Asset::Preload(import_chunk.file.clone()));
                self.iterate_over_chunk_assets(set, import_chunk);
            });
        }
    }
}

#[cfg(test)]
mod test {
    use super::Manifest;

    #[test]
    fn test_generate_html_tags_1() {
        let manifest = Manifest::new("tests/test-manifest.json").unwrap();
        let expected =
        
            r#"<link rel="stylesheet" href="assets/foo-5UjPuW-k.css" />
            <link rel="stylesheet" href="assets/shared-ChJ_j-JJ.css" />
            <script type="module" src="assets/foo-BRBmoGS9.js"></script>
            <link rel="modulepreload" href="assets/shared-B7PI925R.js" />"#;

        let generated = manifest.generate_html_tags(vec!["views/foo.js"]);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_generate_html_tags_2() {
        let manifest = Manifest::new("tests/test-manifest.json").unwrap();
        let expected =
        
            r#"<link rel="stylesheet" href="assets/shared-ChJ_j-JJ.css" />
            <script type="module" src="assets/bar-gkvgaI9m.js"></script>
            <link rel="modulepreload" href="assets/shared-B7PI925R.js" />"#;

        let generated = manifest.generate_html_tags(vec!["views/bar.js"]);

        assert_eq!(expected, generated);
    }
}
