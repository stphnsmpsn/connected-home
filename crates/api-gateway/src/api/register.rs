use self::super::super::schema;
use crate::api::api::make_response;
use crate::users::user::{User, UserRequest};
use diesel::prelude::*;
use diesel::PgConnection;
use hyper::Body;
use schema::users::dsl::*;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::{Response, StatusCode};

pub async fn register(
    new_user: UserRequest,
    pg_connection: Arc<Mutex<PgConnection>>,
) -> Response<Body> {
    let db = pg_connection.lock().await;

    let results = users
        .filter(username.eq(new_user.username.clone()))
        .limit(1)
        .load::<User>(db.deref())
        .expect("Error querying user");

    for _user in results {
        // TODO: handle error: user already exists
        return make_response(StatusCode::BAD_REQUEST, None);
    }

    let hashed_user = User::new(new_user.username, new_user.password);

    // TODO: handle error saving new user
    diesel::insert_into(users)
        .values(&hashed_user)
        .execute(db.deref())
        .unwrap();

    make_response(StatusCode::CREATED, None)
}
