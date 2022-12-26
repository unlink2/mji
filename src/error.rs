use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Command failed")]
    CommandFailed,
    #[error("Unknown error")]
    Unknown,
}
