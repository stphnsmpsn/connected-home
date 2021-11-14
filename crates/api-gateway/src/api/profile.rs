use crate::api::api::make_response;
use crate::users::jwt::Jwt;
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use warp::http::{Method, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize)]
struct Profile {
    first_name: String,
    last_name: String,
    street_number: i32,
    street: String,
    city: String,
    postal_code: String,
}

pub fn profile(method: Method, jwt: Jwt) -> Response<Body> {
    let claims = jwt.verify();
    return match claims {
        Ok(claims) => match method {
            Method::GET => make_response(StatusCode::NOT_IMPLEMENTED, get_profile(&claims)),
            _ => make_response(StatusCode::NOT_IMPLEMENTED, None),
        },
        _ => make_response(StatusCode::UNAUTHORIZED, None),
    };
}

pub fn get_profile(claims: &BTreeMap<String, String>) -> Option<String> {
    debug!("Getting Profile for {}", claims.get("username").unwrap());
    None
}
