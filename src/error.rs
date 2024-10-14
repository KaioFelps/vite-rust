use std::{error::Error, fmt};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ViteErrorKind {
    Manifest
}

#[derive(Debug, Deserialize)]
pub struct ViteError {
    cause: String,
    kind: ViteErrorKind
}

impl ViteError {
    pub fn new(cause: String, kind: ViteErrorKind) -> Self {
        ViteError { cause, kind }
    }
}

impl fmt::Display for ViteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl Error for ViteError {}