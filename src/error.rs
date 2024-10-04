#![allow(dead_code)]

use crate::network::NetworkEvent;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Static error: {0}")]
    Static(&'static str),

    #[error(transparent)]
    Query(#[from] diesel::result::Error),

    #[error(transparent)]
    Connection(#[from] diesel::result::ConnectionError),

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
    SendError(#[from] std::sync::mpsc::SendError<NetworkEvent>),
}
