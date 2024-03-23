#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("parse error {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("io error {0}")]
    IOError(#[from] std::io::Error),

    #[error("missing parameters")]
    MissingParameters,
    #[error("not found")]
    NotFound,
    #[error("transparent")]
    InternalError,
    #[error("unknown data store error")]
    Unknown,
}
