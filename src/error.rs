use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Emoji not found")]
    NotFound,
    #[error("Command failed")]
    CommandFailed,
    #[error("Unknown error")]
    Unknown,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
