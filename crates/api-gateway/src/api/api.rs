use crate::api::countries::countries;
use crate::api::login::login;
use crate::api::profile::profile;
use crate::api::register::register;
use crate::users::jwt::Jwt;
use diesel::PgConnection;
use hyper::Body;
use std::convert::TryInto;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::{Method, Response, StatusCode};

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
    pg_connection: Arc<Mutex<PgConnection>>,
) -> Result<Response<Body>, warp::Rejection> {
    info!(
        "Got {} request for: /api/{} with token: {:?}",
        method, path, jwt
    );
    let jwt = Jwt::from(jwt.unwrap_or_default());
    // todo: find a better way to filter requests, returning appropriate status codes
    Ok(match method {
        Method::POST => match path.as_str() {
            "login" => login(body.as_ref().try_into()?, pg_connection).await,
            "register" => register(body.as_ref().try_into()?, pg_connection).await,
            _ => make_response(StatusCode::BAD_REQUEST, None),
        },
        Method::GET => match path.as_str() {
            "profile" => profile(method, jwt),
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
