#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Asset {
    StyleSheet(String),
    EntryPoint(String),
    Preload(String),
}

impl Asset {
    pub fn to_html(self) -> String {
        match self {
            Self::StyleSheet(file) => {
                return format!("<link rel=\"stylesheet\" href=\"{file}\" />");
            },
            Self::EntryPoint(file) => {
                return format!("<script type=\"module\" src=\"{file}\"></script>");
            },
            Self::Preload(file) => {
                return format!("<link rel=\"modulepreload\" href=\"{file}\" />");
            }
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