use std::sync::Arc;

use axum::{Extension, Json};
use common::{
    auth::jwt::Jwt,
    rest::{ConnectedHomeApiError, ConnectedHomeApiResult},
};
use grpc::{
    user::{user_service_client::UserServiceClient, LoginRequest, UserCredentials},
    SendTracingContext,
};
use http::StatusCode;
use tracing_attributes::instrument;

use crate::context::Context;

// #[debug_handler]
#[instrument(skip(ctx))]
pub(crate) async fn login(
    Extension(ctx): Extension<Arc<Context>>,
    Json(req): Json<LoginUserRequest>,
) -> ConnectedHomeApiResult<LoginUserResponse> {
    // TODO: consider long-lived channels over creating one on each request
    let channel = tonic::transport::Channel::from_shared(ctx.config.remote.user_service.clone())
        .unwrap()
        .connect()
        .await
        .unwrap();

    // creating gRPC client from channel
    let mut client = UserServiceClient::with_interceptor(channel, SendTracingContext::default());

    // creating a new Request
    let request = tonic::Request::new(LoginRequest {
        credentials: Some(UserCredentials {
            username: req.username.clone(),
            password: req.password,
        }),
    });

    // sending request and waiting for response
    let response = client
        .login(request)
        .await
        .map_err(|_| ConnectedHomeApiError::new(StatusCode::UNAUTHORIZED, None))?;

    let login_response = response.into_inner();

    Ok((
        StatusCode::OK,
        Json(LoginUserResponse {
            username: req.username,
            token: Jwt::from(login_response.jwt),
        }),
    ))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoginUserRequest {
    username: String,
    password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoginUserResponse {
    pub username: String,
    pub token: Jwt,
}
