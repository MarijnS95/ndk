use cargo_subcommand::Error as SubcommandError;
use ndk_build::error::NdkError;
use std::io::Error as IoError;
use thiserror::Error;
use toml::de::Error as TomlError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Subcommand(#[from] SubcommandError),
    #[error("Failed to parse config.")]
    Config(#[from] TomlError),
    #[error(transparent)]
    Ndk(#[from] NdkError),
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("When multiple activities are specified in `Cargo.toml` at least one must contain `rust_name=\"{0}\"`")]
    UnspecifiedActivity(String),
}

impl Error {
    pub fn invalid_args() -> Self {
        Self::Subcommand(SubcommandError::InvalidArgs)
    }
}
