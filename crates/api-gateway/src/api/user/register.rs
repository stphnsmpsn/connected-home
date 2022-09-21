use crate::api::make_response;
use crate::api::user::UserRequest;
use grpc::user::user_service_client::UserServiceClient;
use grpc::user::RegisterRequest;
use grpc::user::UserCredentials;
use hyper::Body;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use std::collections::HashMap;
use std::convert::TryFrom;
use tonic::metadata::{AsciiMetadataKey, AsciiMetadataValue};
use tracing::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use warp::http::header::{HeaderName, HeaderValue};
use warp::http::{Response, StatusCode};

#[instrument]
pub async fn register(new_user: UserRequest) -> Response<Body> {
    tracing::info!("attempting to register new user");

    let span = tracing::Span::current();
    let context = span.context();
    let propagator = TraceContextPropagator::new();
    let mut fields = HashMap::new();
    propagator.inject_context(&context, &mut fields);

    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://user-service:8083")
        .connect()
        .await
        .unwrap();

    // creating gRPC client from channel
    let mut client = UserServiceClient::new(channel);

    // creating a new Request
    let mut request = tonic::Request::new(RegisterRequest {
        credentials: Some(UserCredentials {
            username: new_user.username,
            password: new_user.password,
        }),
    });

    let meta = request.metadata_mut();
    for (k, v) in fields.into_iter() {
        let metadata_key =
            AsciiMetadataKey::from_bytes(HeaderName::try_from(k).unwrap().as_str().as_bytes())
                .unwrap();

        let metadata_value =
            AsciiMetadataValue::try_from_bytes(HeaderValue::try_from(v).unwrap().as_bytes())
                .unwrap();

        meta.insert(metadata_key, metadata_value);
    }

    // sending request and waiting for response
    let response = client.register(request).await;

    match response {
        Ok(success_response) => {
            let register_response = success_response.into_inner();
            tracing::info!("successfully registered new user");
            make_response(StatusCode::CREATED, Some(register_response.jwt))
        }
        Err(err_response) => {
            tracing::info!("failed to register new user");
            make_response(
                StatusCode::BAD_REQUEST,
                Some(err_response.message().to_string()),
            )
        }
    }
}
