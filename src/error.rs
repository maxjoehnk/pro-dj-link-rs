use std::io;
use thiserror::Error;

pub type ProDjLinkResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error {0}")]
    IoError(#[from] io::Error),
}
