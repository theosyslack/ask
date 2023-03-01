#[derive(Debug)]
pub enum ErrorKind {
    NoApiKey,
    InvalidApiKey,
    RequestFailed,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ErrorKind::InvalidApiKey => "InvalidApiKey",
            ErrorKind::RequestFailed => "RequestFailed",
            ErrorKind::NoApiKey => "NoApiKey",
        };

        write!(f, "{}", string)
    }
}
