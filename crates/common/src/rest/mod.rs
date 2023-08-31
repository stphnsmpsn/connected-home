use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use std::fmt::Display;

/// type alias that ensures Ok and Err implement IntoResponse
pub type ConnectedHomeApiResult<T> = Result<(StatusCode, Json<T>), ConnectedHomeApiError>;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ConnectedHomeApiError {
    status_code: StatusCode,
    body: ConnectedHomeApiErrorBody,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Hash, Clone)]
pub struct ConnectedHomeApiErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl Display for ConnectedHomeApiErrorBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = self
            .error
            .as_ref()
            .map_or_else(|| "No error message".to_string(), String::from);
        write!(f, "{msg}")
    }
}

impl ConnectedHomeApiError {
    pub fn new(status_code: StatusCode, error: Option<String>) -> Self {
        Self {
            status_code,
            body: ConnectedHomeApiErrorBody { error },
        }
    }
}

impl IntoResponse for ConnectedHomeApiError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.body)).into_response()
    }
}
