use serde::Serialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{roles::Role, keys::Keys};
use jwt::SignWithKey;

#[derive(Serialize)]
pub struct Token {
    pub role: Role,
    pub user_id: u64,
}

pub enum TokenType {
    Access,
    Refresh,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Access => "access",
            Self::Refresh => "refresh",
        }
    }
}

impl Token {
    /// **WARNING:** the key returned by this method should be sent to the client as early as
    /// possible, since **it contains an expiration date**!
    pub fn sign(&self, r#type: TokenType, keys: Keys) -> String {
        #[derive(Serialize)]
        struct ActualToken {
            sub: u64,
            exp: u64,
            r#type: &'static str,
            role: &'static str,
        }
        let key = match self.role {
            Role::Admin => keys.admin(),
            Role::Cashier => keys.cashier(),
        };
        let full_token = ActualToken {
            exp: {
                let expiration_time = (SystemTime::now() + {
                    let expiration_duration = Duration::from_secs(60 * 10);
                    expiration_duration
                })
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
                expiration_time
            },
            sub: self.user_id,
            r#type: r#type.as_str(),
            role: self.role.as_str(),
        };
        full_token.sign_with_key(&key.inner()).unwrap()
    }
}
