use chrono::{DateTime, Utc};
use hmac::{Hmac, NewMac};
use jwt::{Error, VerifyWithKey};
use jwt::{Header, Token};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, Serialize)]
pub enum JwtError {
    INVALID,
    EXPIRED,
}

impl From<jwt::Error> for JwtError {
    fn from(error: Error) -> Self {
        match error {
            _ => JwtError::INVALID,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Jwt {
    pub token: String,
}

impl From<String> for Jwt {
    fn from(token: String) -> Self {
        Self { token }
    }
}

impl Default for Jwt {
    fn default() -> Self {
        Self {
            token: String::default(),
        }
    }
}

impl Display for Jwt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self)
                .map_err(|e| { warn!("Failed to serialize JWT with error: {}", e) })
                .unwrap()
        )
    }
}

impl Jwt {
    pub fn verify(&self, key: &str) -> Result<BTreeMap<String, String>, JwtError> {
        // todo: manage secrets
        let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();

        let token: Token<Header, BTreeMap<String, String>, _> =
            VerifyWithKey::verify_with_key(self.token.as_str(), &key)?;
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
