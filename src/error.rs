use serde::Deserialize;
use std::{error::Error, fmt};

#[derive(Debug, Deserialize)]
pub enum ViteErrorKind {
    Manifest,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ViteError {
    cause: Box<str>,
    kind: ViteErrorKind,
}

impl ViteError {
    pub fn new<T>(cause: T, kind: ViteErrorKind) -> Self
    where
        T: ToString,
    {
        ViteError {
            cause: cause.to_string().into_boxed_str(),
            kind,
        }
    }
}

impl fmt::Display for ViteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl Error for ViteError {}
