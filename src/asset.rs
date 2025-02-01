#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Asset {
    StyleSheet(String),
    EntryPoint(String),
    Preload(String),
}

enum PreloadAsset {
    JavaScript,
    Image,
    Font,
    Video,
    Audio,
    Css,
    Unknown,
}

impl Asset {
    pub fn style_sheet(file: String, prefix: Option<&str>, app_url: &str) -> Self {
        Self::StyleSheet(Self::resolve_asset_path(file, prefix, app_url))
    }

    pub fn entry_point(file: String, prefix: Option<&str>, app_url: &str) -> Self {
        Self::EntryPoint(Self::resolve_asset_path(file, prefix, app_url))
    }

    pub fn pre_load(file: String, prefix: Option<&str>, app_url: &str) -> Self {
        Self::Preload(Self::resolve_asset_path(file, prefix, app_url))
    }

    pub fn into_html(self) -> String {
        match self {
            Self::StyleSheet(file) => {
                format!(r#"<link rel="stylesheet" href="{file}" />"#)
            }
            Self::EntryPoint(file) => {
                format!(r#"<script type="module" src="{file}"></script>"#)
            }
            Self::Preload(file) => match Asset::get_file_type(&file) {
                PreloadAsset::JavaScript => {
                    format!(r#"<link rel="modulepreload" href="{file}" />"#)
                }
                PreloadAsset::Css => {
                    format!(r#"<link rel="preload" as="style" href="{file}" />"#)
                }
                PreloadAsset::Audio => {
                    format!(r#"<link rel="preload" as="font" href="{file}" />"#)
                }
                PreloadAsset::Font => {
                    format!(r#"<link rel="preload" as="audio" href="{file}" />"#)
                }
                PreloadAsset::Image => {
                    format!(r#"<link rel="preload" as="image" href="{file}" />"#)
                }
                PreloadAsset::Video => {
                    format!(r#"<link rel="preload" as="video" href="{file}" />"#)
                }
                PreloadAsset::Unknown => "".into(),
            },
        }
    }

    fn get_file_type(file: &str) -> PreloadAsset {
        if file.ends_with(".js") {
            return PreloadAsset::JavaScript;
        }
        if file.ends_with(".css") {
            return PreloadAsset::Css;
        }

        if file.ends_with(".png")
            || file.ends_with(".jpg")
            || file.ends_with(".gif")
            || file.ends_with(".svg")
            || file.ends_with(".webp")
        {
            return PreloadAsset::Image;
        }

        if file.ends_with(".woff")
            || file.ends_with(".woff2")
            || file.ends_with(".ttf")
            || file.ends_with(".eot")
        {
            return PreloadAsset::Font;
        }

        if file.ends_with(".mp4") || file.ends_with(".webm") || file.ends_with(".mov") {
            return PreloadAsset::Video;
        }

        if file.ends_with(".mp3")
            || file.ends_with(".wav")
            || file.ends_with(".aac")
            || file.ends_with(".m4a")
        {
            return PreloadAsset::Audio;
        }

        PreloadAsset::Unknown
    }

    pub(crate) fn resolve_asset_path(file: String, prefix: Option<&str>, app_url: &str) -> String {
        println!("{file}");
        println!("{prefix:#?}");
        println!("{app_url}");
        match prefix {
            Some(prefix) => format!("{app_url}/{prefix}/{file}"),
            None => format!("{app_url}/{file}"),
        }
    }
}

impl std::hash::Hash for Asset {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        match self {
            Asset::EntryPoint(v) => v.hash(hasher),
            Asset::Preload(v) => v.hash(hasher),
            Asset::StyleSheet(v) => v.hash(hasher),
        }
    }
}
