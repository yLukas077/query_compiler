use thiserror::Error;

/// Error types for parsing `.query` files.
#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("Syntax error: {0}")]
    Expected(String),
}
