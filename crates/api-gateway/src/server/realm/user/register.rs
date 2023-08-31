use crate::context::Context;
use axum::{Extension, Json};
use common::{
    auth::jwt::Jwt,
    rest::{ConnectedHomeApiError, ConnectedHomeApiResult},
};
use grpc::{
    user::{user_service_client::UserServiceClient, RegisterRequest, UserCredentials},
    SendTracingContext,
};
use http::StatusCode;
use std::sync::Arc;
use tracing_attributes::instrument;

// #[debug_handler]
#[instrument(skip(ctx))]
pub(crate) async fn register(
    Extension(ctx): Extension<Arc<Context>>,
    Json(req): Json<RegisterUserRequest>,
) -> ConnectedHomeApiResult<RegisterUserResponse> {
    // TODO: consider long-lived channels over creating one on each request
    let channel = tonic::transport::Channel::from_shared(ctx.config.remote.user_service.clone())
        .unwrap()
        .connect()
        .await
        .unwrap();

    // creating gRPC client from channel
    let mut client = UserServiceClient::with_interceptor(channel, SendTracingContext::default());

    // creating a new Request
    let request = tonic::Request::new(RegisterRequest {
        credentials: Some(UserCredentials {
            username: req.username.clone(),
            password: req.password,
        }),
    });

    // sending request and waiting for response
    let response = client
        .register(request)
        .await
        .map_err(|_| ConnectedHomeApiError::new(StatusCode::INTERNAL_SERVER_ERROR, None))?;

    let register_response = response.into_inner();

    Ok((
        StatusCode::OK,
        Json(RegisterUserResponse {
            username: req.username,
            token: Jwt::from(register_response.jwt),
        }),
    ))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterUserResponse {
    pub username: String,
    pub token: Jwt,
}
