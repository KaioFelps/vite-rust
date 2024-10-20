use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use md5::{Digest, Md5};
use serde::Deserialize;


use crate::asset::Asset;
use crate::chunk::Chunk;
use crate::error::{ViteError, ViteErrorKind};

#[derive(Deserialize, Debug)]
pub(crate) struct Manifest {
    manifest: HashMap<String, Chunk>,
    hash: String
}

impl Manifest {
    pub fn new(path: &str) -> Result<Self, ViteError> {
        let mut file = match File::open(path) {
            Err(err) => {
                return Err(ViteError::new(
                    format!("Failed to open manifest at {}: {}", path, err),
                    ViteErrorKind::Manifest
                ));
            },
            Ok(file) => file,
        };

        let mut manifest_content= String::new();
        if let Err(err) = file.read_to_string(&mut manifest_content) {
            return Err(ViteError::new(
                format!("Failed to read manifest.json content: {}", err),
                ViteErrorKind::Manifest
            ));
        };
        
        let hash = match Manifest::get_hash_from_manifest(&manifest_content) {
            Ok(hash) => hash,
            Err(err) => {
                return Err(ViteError::new(
                    format!("Failed to generate hash for manifest: {err}"),
                    ViteErrorKind::Manifest
                ))
            }
        };

        let manifest = serde_json::from_str(&manifest_content);

        return match manifest {
            Err(err) => Err(ViteError::new(
                format!("Failed to parse manifest json: {}", err),
                ViteErrorKind::Manifest
            )),
            Ok(manifest) => Ok(Manifest {
                manifest,
                hash,
            })
        };
    }

    fn get_hash_from_manifest(content: &str) -> Result<String, std::io::Error> {        
        let mut hasher = Md5::new();
        hasher.update(&content.as_bytes());
        let hash = hasher.finalize();

        let mut buffer = Vec::new();

        for byte in hash.bytes() {
            buffer.push(byte?);
        }

        Ok(hex::encode(&buffer))
    }

    #[inline]
    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn generate_html_tags(&self, entrypoints: &Vec<String>) -> String {
        if self.manifest.is_empty() {
            log::error!("Manifest is empty. Empty string being returned from `Manifest::generate_html_tags`.");
            return "".into();
        }

        let mut discovered_assets = HashSet::<Asset>::new();

        for entry in entrypoints {
            let entry_chunk = match self.manifest.get(entry) {
                None => {
                    log::error!(r#"Skipping invalid or unexisting entry "{entry}"."#);
                    continue;
                },
                Some(chunk) => chunk,
            };

            let entry_as_asset = if entry.ends_with(".css") {
                Asset::StyleSheet(entry_chunk.file.clone())
            } else {
                Asset::EntryPoint(entry_chunk.file.clone())
            };

            if !discovered_assets.contains(&entry_as_asset) {
                discovered_assets.insert(entry_as_asset);
                self.iterate_over_chunk_assets(&mut discovered_assets, entry_chunk);
            }
        }

        let mut assets = discovered_assets.into_iter().collect::<Vec<Asset>>();
        // Puts the assets in the following order: stylesheets > entries > preloads
        assets.sort();

        return assets
            .into_iter()
            .map(|asset| asset.to_html())
            .collect::<Vec<String>>()
            .join("\n");
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

    /// Generates a list of keys of every chunk that `isEntry`.
    pub(crate) fn get_manifest_entries<'a>(&'a self) -> Vec<&'a str> {
        let mut entries = Vec::new();

        for (key, chunk) in self.manifest.iter() {
            if chunk.is_entry {
                entries.push(key.as_str());
            }
        }

        return entries;
    }
}

#[cfg(test)]
mod test {
    use super::Manifest;
    use crate::test_utils::NormalizeHtmlStrings;

    #[test]
    fn test_generate_html_tags_1() {
        let manifest = Manifest::new("tests/test-manifest.json").unwrap();
        let expected =
        
            r#"<link rel="stylesheet" href="assets/foo-5UjPuW-k.css" />
            <link rel="stylesheet" href="assets/shared-ChJ_j-JJ.css" />
            <script type="module" src="assets/foo-BRBmoGS9.js"></script>
            <link rel="modulepreload" href="assets/shared-B7PI925R.js" />"#
            .__normalize_html_strings();

        let generated = manifest.generate_html_tags(&vec!["views/foo.js".into()]);

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_generate_html_tags_2() {
        let manifest = Manifest::new("tests/test-manifest.json").unwrap();
        let expected =
            r#"<link rel="stylesheet" href="assets/shared-ChJ_j-JJ.css" />
            <script type="module" src="assets/bar-gkvgaI9m.js"></script>
            <link rel="modulepreload" href="assets/shared-B7PI925R.js" />"#
            .__normalize_html_strings();

        let generated = manifest.generate_html_tags(&vec!["views/bar.js".into()]);

        assert_eq!(expected, generated);
    }
}
