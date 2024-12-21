use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HelmWrapperError {
    #[error("Helm command execution error")]
    ExecutionError(#[from] std::io::Error),

    #[error("Helm command execution error")]
    Error,
}
