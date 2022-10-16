use std::fmt::{Display, Formatter};

pub type UrlParseResult<T> = Result<T, UrlParseError>;

#[derive(Debug, thiserror::Error)]
pub enum UrlParseError {
    NoPath,
    NotHttps,
    Parser(url::ParseError),
}

impl Display for UrlParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlParseError::NoPath => write!(f, "URL path is missing."),
            UrlParseError::NotHttps => write!(f, "The URL protocol should be https."),
            UrlParseError::Parser(e) => write!(f, "Error while parsing the URL: {}", e),
        }
    }
}

pub type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    NotJSON,
    NoUTF8,
    NetworkError,
    SerializeError,
    NotFoundOrNullBody,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::NotJSON => write!(f, "Invalid JSON"),
            RequestError::NoUTF8 => write!(f, "Utf8 error"),
            RequestError::NetworkError => write!(f, "Network error"),
            RequestError::SerializeError => write!(f, "Serialize error"),
            RequestError::NotFoundOrNullBody => write!(f, "Body is null or record is not found"),
        }
    }
}
