use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("stream ended unexpectedly after {offset} bytes")]
    UnexpectedEof { offset: u64 },
    #[error("not an R serialization stream (starts with {0:?})")]
    NotSerialized(Vec<u8>),
    #[error("unsupported {what} at {path} (byte {offset})")]
    Unsupported {
        what: String,
        path: String,
        offset: u64,
    },
    #[error("malformed stream at {path} (byte {offset}): {msg}")]
    Malformed {
        msg: String,
        path: String,
        offset: u64,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
