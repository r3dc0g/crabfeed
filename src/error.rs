#![allow(dead_code)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Static error: {0}")]
    Static(&'static str),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Query(#[from] diesel::result::Error),

    #[error(transparent)]
    Connection(#[from] diesel::result::ConnectionError),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),
}
