use async_trait::async_trait;
use common::error::ConnectedHomeError;
use sqlx::{pool::PoolConnection, Postgres};

use crate::repo::{Repo, UserDto};

#[async_trait]
impl<T> Repo for T
where
    T: Send + Sync,
    for<'a> &'a mut T: sqlx::Executor<'a, Database = Postgres>,
{
    type Error = ConnectedHomeError;
    type Connection = PoolConnection<Postgres>;

    async fn load_user(&mut self, username: String) -> Result<Option<UserDto>, Self::Error> {
        let query = sqlx::query_as!(
            UserDto,
            // language=PostgreSQL
            r#"
            SELECT 
                id, 
                username, 
                password, 
                created_at, 
                updated_at, 
                locked
            FROM users
            WHERE username = $1
            "#,
            username
        );

        let result = query.fetch_optional(self).await;

        Ok(result?)
    }

    async fn store_user(&mut self, username: String, password: String) -> Result<UserDto, Self::Error> {
        let query = sqlx::query_as!(
            UserDto,
            // language=PostgreSQL
            r#"
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            RETURNING *;
            "#,
            username,
            password,
        );

        query.fetch_one(self).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => ConnectedHomeError::UserAlreadyExists(username),
            _ => ConnectedHomeError::Sqlx(e),
        })
    }
}
