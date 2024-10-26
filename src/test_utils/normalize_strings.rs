pub(crate) trait NormalizeHtmlStrings {
    fn __normalize_html_strings(self) -> String;
}

impl<T: ToString> NormalizeHtmlStrings for T {
    fn __normalize_html_strings(self) -> String {
        self
        .to_string()
        .replace("\t", "     ")
        .lines()
        .map(str::trim)
        .collect::<Vec::<&str>>()
        .join("\n")
        .trim()
        .to_string()
    }
}