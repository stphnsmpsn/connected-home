use axum::{
    routing::{get, post},
    Router,
};

mod login;
mod profile;
mod register;

pub fn router() -> Router {
    Router::new()
        .route("/user/register", post(register::register))
        .route("/user/login", post(login::login))
        .route("/user/profile", get(profile::profile))
}
