#![allow(unused)]
use crate::{
    asset::Asset,
    manifest::{self, Manifest},
};
use serde::Deserialize;

// All strings are Strings instead of &'lf str
// because serde can't borrow data from the Deserializer
// when its deserializing from an IO reader
// such as file reader (which we use to parse the manifest).
//
// Hence, we cannot borrow strings, only own them.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Chunk {
    /// The path for the bundled file (relative to the output directory).
    pub(crate) file: String,

    /// The path to the original file.
    #[serde(default)]
    pub(crate) src: Option<String>,

    /// The name of the original file (without extension).
    #[serde(default)]
    pub(crate) name: Option<String>,

    /// If chunk is an entry.
    #[serde(default)]
    pub(crate) is_entry: bool,

    /// If chunk is imported by an entry.
    #[serde(default)]
    pub(crate) is_dynamic_entry: bool,

    /// If chunk is implicited required by an entry.
    #[serde(default)]
    pub(crate) is_implicit_entry: bool,

    /// If chunk is part of a legacy bundle (generated by
    /// legacy plugin).
    #[serde(default)]
    pub(crate) is_legacy_entry: bool,

    /// Some hash for verifying the asset's integrity.
    /// Present if Subresource Integrity is enabled.
    #[serde(default)]
    pub(crate) integrity: Option<String>,

    /// List of files (`Chunk::file`) imported by this one.
    #[serde(default)]
    pub(crate) imports: Vec<String>,

    /// List of files (`Chunk::file`) lazily-imported by this one.
    #[serde(default)]
    pub(crate) dynamic_imports: Vec<String>,

    /// List of css files used by this chunk.
    #[serde(default)]
    pub(crate) css: Vec<String>,

    /// List of assets imported by this chunk, e.g. images.
    #[serde(default)]
    pub(crate) assets: Vec<String>,
}

#[derive(PartialEq)]
enum ChunkIterListTrack {
    Assets,
    Css,
    Eot, // end-of-tracking
}

impl ChunkIterListTrack {
    pub fn start() -> Self {
        Self::Assets
    }
}

struct ChunkIter<'a> {
    assets: &'a [String],
    imports: &'a [String],
    index: usize,
    css: &'a [String],
    track: ChunkIterListTrack,
    prefix: Option<&'a str>,
}

impl Chunk {
    /// Returns an [`Iterator<Item = Asset>`], where the returned assets
    /// are the Chunk's `assets`, `imports` and `css` fields, respectively.
    pub fn assets_iter(&self, prefix: Option<&'static str>) -> impl Iterator<Item = Asset> + '_ {
        ChunkIter {
            assets: &self.assets,
            imports: &self.imports,
            css: &self.css,
            index: 0,
            track: ChunkIterListTrack::start(),
            prefix,
        }
    }
}

impl Iterator for ChunkIter<'_> {
    type Item = Asset;

    fn next(&mut self) -> Option<Self::Item> {
        if self.track == ChunkIterListTrack::Assets {
            if let Some(asset) = self.assets.get(self.index) {
                self.index += 1;
                return Some(Asset::pre_load(asset.clone(), self.prefix));
            } else {
                self.track = ChunkIterListTrack::Css;
                self.index = 0;
            }
        }

        if self.track == ChunkIterListTrack::Css {
            if let Some(css) = self.css.get(self.index) {
                self.index += 1;
                return Some(Asset::style_sheet(css.clone(), self.prefix));
            } else {
                self.track = ChunkIterListTrack::Eot;
                self.index = 0;
            }
        }

        None
    }
}
