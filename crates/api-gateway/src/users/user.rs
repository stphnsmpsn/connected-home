use self::super::super::schema::users;
use crate::api::api::DeserializationError;
use argon2::{self, Config};
use chrono::{NaiveDateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    username: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    locked: bool,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        let now = Utc::now().naive_utc();
        User {
            username,
            password: hash(password.as_bytes()),
            created_at: now,
            updated_at: now,
            locked: false,
        }
    }

    pub fn verify_password(&self, password: String) -> bool {
        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap_or(false)
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

impl TryFrom<&[u8]> for UserRequest {
    type Error = DeserializationError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(bytes).map_err(|_| DeserializationError::InvalidRequestBody)
    }
}

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
