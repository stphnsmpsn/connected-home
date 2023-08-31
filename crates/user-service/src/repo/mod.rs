use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

pub mod pg;

#[async_trait]
pub trait Repo {
    type Error;
    type Connection;

    async fn load_user(&mut self, username: String) -> Result<Option<UserDto>, Self::Error>;
    async fn store_user(&mut self, username: String, password: String) -> Result<UserDto, Self::Error>;
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub locked: bool,
}

impl UserDto {
    #[tracing::instrument]
    pub fn verify_password<T>(&self, password: T) -> bool
    where
        T: AsRef<str> + std::fmt::Debug,
    {
        argon2::verify_encoded(&self.password, password.as_ref().as_bytes()).unwrap_or(false)
    }
}
