use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseError {
    #[error("`{0}` is empty or missing")]
    CheckFailed(String),
    #[error("`{0}` has an invalid format (expected format is `{1}`)")]
    InvalidFormat(String, String),
}
