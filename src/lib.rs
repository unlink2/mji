#![feature(error_generic_member_access)]
#![feature(provide_any)]

pub mod config;
pub mod error;
pub mod gitmoji;
pub mod mjimap;
pub mod prompt;

pub use config::{Config, CFG};
pub use error::Error;
