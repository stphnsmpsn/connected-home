use self::super::super::schema;
use crate::api::api::make_response;
use crate::users::jwt::Jwt;
use crate::users::user::{User, UserRequest};
use diesel::prelude::*;
use diesel::PgConnection;
use hyper::Body;
use schema::users::dsl::*;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::{Response, StatusCode};

pub async fn login(
    credentials: UserRequest,
    pg_connection: Arc<Mutex<PgConnection>>,
) -> Response<Body> {
    let db = pg_connection.lock().await;

    let results = users
        .filter(username.eq(credentials.username.clone()))
        .limit(1)
        .load::<User>(db.deref())
        .expect("Error querying user");

    for user in results {
        return if user.verify_password(credentials.password) {
            make_response(StatusCode::OK, Some(Jwt::create(&user).to_string()))
        } else {
            make_response(StatusCode::UNAUTHORIZED, None)
        };
    }

    make_response(StatusCode::UNAUTHORIZED, None)
}
