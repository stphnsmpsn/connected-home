pub mod login;
pub mod profile;
pub mod register;

use crate::api::DeserializationError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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
