use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Page out of bounds")]
    PageOutOfBounds,

    #[error("JSON error")]
    Json(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}
