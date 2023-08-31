use std::sync::Arc;

use axum::{Extension, Json};
use common::{
    auth::jwt::Jwt,
    rest::{ConnectedHomeApiError, ConnectedHomeApiResult},
};
use http::StatusCode;
use tracing_attributes::instrument;

use crate::context::Context;

// #[debug_handler]
#[instrument(skip(ctx, jwt))]
pub(crate) async fn profile(
    Extension(jwt): Extension<Option<Jwt>>,
    Extension(ctx): Extension<Arc<Context>>,
    Json(_req): Json<ProfileRequest>,
) -> ConnectedHomeApiResult<ProfileResponse> {
    info!(ctx.logger, "{:?}", jwt);
    Err(ConnectedHomeApiError::new(StatusCode::NOT_IMPLEMENTED, None))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ProfileRequest {}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ProfileResponse {}
