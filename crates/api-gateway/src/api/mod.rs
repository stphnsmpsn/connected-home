use crate::api::countries::countries;
use crate::api::user::login::login;
use crate::api::user::profile::profile;
use crate::api::user::register::register;
use hyper::Body;
use std::convert::TryInto;
use types::jwt::Jwt;
use warp::http::{Method, Response, StatusCode};

mod countries;
mod user;

#[derive(Debug)]
pub enum DeserializationError {
    InvalidRequestBody,
}

impl warp::reject::Reject for DeserializationError {}

pub async fn api(
    method: Method,
    path: String,
    body: hyper::body::Bytes,
    jwt: Option<String>,
) -> Result<Response<Body>, warp::Rejection> {
    info!(
        "Got {} request for: /api/{} with token: {:?}",
        method, path, jwt
    );
    let jwt = Jwt::from(jwt.unwrap_or_default());
    // todo: find a better way to filter requests, returning appropriate status codes
    Ok(match method {
        Method::POST => match path.as_str() {
            "login" => login(body.as_ref().try_into()?).await,
            "register" => register(body.as_ref().try_into()?).await,
            _ => make_response(StatusCode::BAD_REQUEST, None),
        },
        Method::GET => match path.as_str() {
            "profile" => profile(jwt).await,
            "countries" => countries().await,
            _ => make_response(StatusCode::BAD_REQUEST, None),
        },
        _ => make_response(StatusCode::BAD_REQUEST, None),
    })
}

pub fn make_response(status_code: StatusCode, response_body: Option<String>) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(Body::from(response_body.unwrap_or(String::new())))
        .unwrap()
}
