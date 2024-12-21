use thiserror::Error;

#[derive(Error, Debug)]
pub enum HelmWrapperError {
    #[error("Unable to understand helm response format")]
    DeserializationError(#[from] serde_json::Error),

    #[error("Unsupported helm non-utf8 output")]
    NonUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("Helm command execution error")]
    ExecutionError(#[from] std::io::Error),

    #[error("Helm command execution error")]
    Error,
}
