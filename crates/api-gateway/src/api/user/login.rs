use crate::api::make_response;
use crate::api::user::UserRequest;

use grpc::user::user_service_client::UserServiceClient;
use grpc::user::LoginRequest;
use grpc::user::UserCredentials;

use hyper::Body;
use warp::http::{Response, StatusCode};

pub async fn login(new_user: UserRequest) -> Response<Body> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://user-service:8083")
        .connect()
        .await
        .unwrap();

    // creating gRPC client from channel
    let mut client = UserServiceClient::new(channel);

    // creating a new Request
    let request = tonic::Request::new(LoginRequest {
        credentials: Some(UserCredentials {
            username: new_user.username,
            password: new_user.password,
        }),
    });

    // sending request and waiting for response
    let response = client.login(request).await;
    match response {
        Ok(success_response) => {
            let login_response = success_response.into_inner();
            make_response(StatusCode::OK, Some(login_response.jwt))
        }
        Err(err_response) => make_response(
            StatusCode::BAD_REQUEST,
            Some(err_response.message().to_string()),
        ),
    }
}
