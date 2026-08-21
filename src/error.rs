use crate::history::HistoryError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum EzcurlError {
    InvalidUrl(#[from] url::ParseError),
    #[error("invalid header line: {0}")]
    InvalidHeader(String),
    Network(#[from] reqwest::Error),
    History(#[from] HistoryError),
    Terminal(#[from] std::io::Error),
}
