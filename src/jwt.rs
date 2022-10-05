use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rocket::serde::Serialize;

use crate::role::Role;

pub enum TokenType {
    Access,
    Refresh,
}

impl TokenType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Access => "access",
            Self::Refresh => "refresh",
        }
    }
}

#[derive(Deserialize)]
pub struct ReceivedClaims {
    pub sub: u128,
    pub exp: u64,
    pub r#type: String,
    pub role: String,
}

pub fn make_token(user_id: u128, role: &Role, r#type: &TokenType) -> String {
    #[derive(Serialize)]
    pub struct ClaimsToSend<'a> {
        pub sub: u128,
        pub exp: u64,
        pub r#type: &'a str,
        pub role: &'a str,
    }
    jsonwebtoken::encode(
        &Header::default(),
        &ClaimsToSend {
            sub: user_id,
            exp: (SystemTime::now() + Duration::from_secs(60 * 10))
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            r#type: r#type.as_str(),
            role: role.as_str(),
        },
        &EncodingKey::from_secret(std::env::var("JWT_PRIVATE_KEY").unwrap().as_bytes()),
    )
    .unwrap()
}

pub fn decode(token: &str) -> jsonwebtoken::errors::Result<ReceivedClaims> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_PUBLIC_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )?
    .claims)
}
