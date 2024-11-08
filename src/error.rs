use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse data: {0}")]
    ParseError(String),
}
