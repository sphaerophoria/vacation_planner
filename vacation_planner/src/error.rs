use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error
{
    #[error("Http request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Deserializing failed: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Parsing date failed: {0}")]
    DateParse(#[from] chrono::format::ParseError),
    #[error("Failed to parse province ID")]
    ProvinceParse,
    #[error("Insertion failed")]
    InsertionFail,
}
