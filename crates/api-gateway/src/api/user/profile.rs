use crate::api::make_response;
use grpc::user::user_service_client::UserServiceClient;
use grpc::user::{ProfileRequest, ProfileResponse};
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tonic::{metadata::MetadataValue, Request};
use types::jwt::Jwt;
use warp::http::{Response, StatusCode};

pub async fn profile(jwt: Jwt) -> Response<Body> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://user-service:8083")
        .connect()
        .await
        .unwrap();

    let token = MetadataValue::from_str(jwt.to_string().as_str()).unwrap();

    // creating gRPC client from channel
    let mut client = UserServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    // creating a new Request
    let request = tonic::Request::new(ProfileRequest {});

    // sending request and waiting for response
    let response = client.profile(request).await;
    match response {
        Ok(success_response) => {
            let profile_response = success_response.into_inner();
            let profile = Profile::try_from(profile_response).unwrap();
            make_response(
                StatusCode::OK,
                Some(serde_json::to_string(&profile).unwrap()),
            )
        }
        Err(err_response) => make_response(
            StatusCode::BAD_REQUEST,
            Some(err_response.message().to_string()),
        ),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Profile {
    first_name: String,
    last_name: String,
    street_number: i32,
    street: String,
    city: String,
    postal_code: String,
}

impl TryFrom<ProfileResponse> for Profile {
    type Error = ();

    fn try_from(value: ProfileResponse) -> Result<Self, Self::Error> {
        let profile = value.profile.unwrap();

        Ok(Self {
            first_name: profile.first_name,
            last_name: profile.last_name,
            street_number: profile.street_number,
            street: profile.street,
            city: profile.city,
            postal_code: profile.postal_code,
        })
    }
}
