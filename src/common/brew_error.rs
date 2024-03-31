use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BrewError {
    /// file_name => error from serde
    #[error("{0}")]
    FailedToParseYml(#[from] serde_yaml::Error),
    /// file_name => error from a loader
    #[error("[Invalid data error] Failed to load `{0}` <= {1}")]
    FailedToLoad(String, String),
    /// file_name => error message raised by parsers
    #[error("[Syntax error] Failed to parse `{0}` <= {1}")]
    FailedToParse(String, String),
}
