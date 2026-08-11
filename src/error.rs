use std::fmt;

#[derive(Debug)]
pub enum EzCurlError {
    MissingUrl,
    InvalidHeader(String),
    Network(reqwest::Error),
    History(crate::history::HistoryError),
    Terminal(std::io::Error),
}

impl From<reqwest::Error> for EzCurlError {
    fn from(error: reqwest::Error) -> Self {
        Self::Network(error)
    }
}

impl From<std::io::Error> for EzCurlError {
    fn from(error: std::io::Error) -> Self {
        Self::Terminal(error)
    }
}

impl From<crate::history::HistoryError> for EzCurlError {
    fn from(error: crate::history::HistoryError) -> Self {
        Self::History(error)
    }
}

impl fmt::Display for EzCurlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EzCurlError::MissingUrl => f.write_str("Missing URL"),
            EzCurlError::InvalidHeader(header) => write!(f, "invalid header line: {header}"),
            EzCurlError::Network(error) => write!(f, "network error: {error}"),
            EzCurlError::History(error) => write!(f, "history error: {error}"),
            EzCurlError::Terminal(error) => write!(f, "Terminal error: {error}"),
        }
    }
}
