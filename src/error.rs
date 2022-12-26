use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not found!")]
    NotFound,
    #[error("Unknown error")]
    Unknown,
}
