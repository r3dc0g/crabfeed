use std::sync::mpsc;

use crate::app::AppEvent;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Static error: {0}")]
    Static(&'static str),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    SendError(#[from] mpsc::SendError<AppEvent>),

    #[error(transparent)]
    NextRecvError(#[from] tokio::sync::mpsc::error::TryRecvError),

    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Recv(#[from] std::sync::mpsc::RecvError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    FeedParsing(#[from] feed_rs::parser::ParseFeedError),

    #[error(transparent)]
    HTMLParsing(#[from] html_parser::Error),

    #[error(transparent)]
    RecvTimeout(#[from] std::sync::mpsc::RecvTimeoutError),

    #[error(transparent)]
    ConfigurationError(#[from] config::ConfigError),
}
