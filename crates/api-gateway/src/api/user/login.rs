use crate::api::make_response;
use crate::api::user::UserRequest;

use grpc::user::user_service_client::UserServiceClient;
use grpc::user::LoginRequest;
use grpc::user::UserCredentials;
use grpc::SendTracingContext;
use hyper::Body;
use tracing::instrument;
use warp::http::{Response, StatusCode};

#[instrument]
pub async fn login(user: UserRequest) -> Response<Body> {
    tracing::info!("attempting to login");

    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://user-service:8083")
        .connect()
        .await
        .unwrap();

    // creating gRPC client from channel
    let mut client = UserServiceClient::with_interceptor(channel, SendTracingContext::default());

    // creating a new Request
    let request = tonic::Request::new(LoginRequest {
        credentials: Some(UserCredentials {
            username: user.username,
            password: user.password,
        }),
    });

    // sending request and waiting for response
    let response = client.login(request).await;
    match response {
        Ok(success_response) => {
            let login_response = success_response.into_inner();
            tracing::info!("successfully logged in");
            make_response(StatusCode::OK, Some(login_response.jwt))
        }
        Err(err_response) => {
            tracing::info!("failed to log in");
            make_response(
                StatusCode::BAD_REQUEST,
                Some(err_response.message().to_string()),
            )
        }
    }
}
