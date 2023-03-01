mod kind;

use std::env::VarError;

pub use kind::ErrorKind;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.kind)
    }
}

impl std::error::Error for Error {}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        let kind = match value {
            VarError::NotPresent => ErrorKind::NoApiKey,
            VarError::NotUnicode(_) => ErrorKind::InvalidApiKey,
        };

        Error {
            kind,
            message: None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error {
            kind: ErrorKind::RequestFailed,
            message: Some(value.to_string()),
        }
    }
}
