use std::{net::AddrParseError, string::FromUtf8Error};

use http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectedHomeError {
    #[error("{0}")]
    Unspecified(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Prometheus(#[from] prometheus::Error),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    #[error(transparent)]
    ParseAddr(#[from] AddrParseError),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    TonicTransport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("A user with that username: {0}, already exists")]
    UserAlreadyExists(String),
    #[error("TODO: MQTT ERROR")]
    Mqtt,
}

impl ConnectedHomeError {
    pub fn status_code(&self) -> StatusCode {
        // match self {
        //     _ => StatusCode::INTERNAL_SERVER_ERROR,
        // }
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[allow(unused)]
pub type ConnectedHomeResult<T> = Result<T, ConnectedHomeError>;
