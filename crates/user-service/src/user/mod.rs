use argon2::{self, Config};
use chrono::{NaiveDateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use self::super::schema::users;

pub mod handlers;

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

    pub fn username(&self) -> &str {
        self.username.as_str()
    }
}

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
