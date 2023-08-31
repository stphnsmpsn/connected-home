use chrono::{DateTime, Utc};
use hmac::{digest::KeyInit, Hmac};
use http::{header::ToStrError, HeaderValue};
use jwt::{Error, Header, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

#[derive(Debug, Deserialize, Serialize)]
pub enum JwtError {
    INVALID,
    EXPIRED,
}

impl From<jwt::Error> for JwtError {
    fn from(_: Error) -> Self {
        JwtError::INVALID
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Jwt {
    pub token: String,
}

impl TryFrom<&HeaderValue> for Jwt {
    type Error = ToStrError;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        Ok(Jwt::from(value.to_str()?.to_string()))
    }
}

impl From<String> for Jwt {
    fn from(token: String) -> Self {
        Self { token }
    }
}

impl Display for Jwt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl Jwt {
    pub fn verify(&self, key: &str) -> Result<BTreeMap<String, String>, JwtError> {
        // todo: manage secrets
        let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();

        let token: Token<Header, BTreeMap<String, String>, _> = VerifyWithKey::verify_with_key(
            match self.token.find("Bearer ") {
                Some(0) => &self.token[7..],
                _ => &self.token,
            },
            &key,
        )?;

        let expiry = token.claims().get("expiry");
        return match expiry {
            Some(expiry) => {
                let expiry = DateTime::parse_from_str(expiry, "%Y-%m-%d %H:%M:%S %z").unwrap();
                if expiry < Utc::now() {
                    return Err(JwtError::EXPIRED);
                }
                Ok(token.claims().clone())
            }
            _ => Err(JwtError::INVALID),
        };
    }
}
