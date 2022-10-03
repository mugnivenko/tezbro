use std::str::FromStr;

use serde::Serialize;

pub enum Role {
    Admin,
    Cashier,
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Cashier => "cashier",
        }
    }
}

/// The provided string is not a role name in lowercase.
pub struct StringIsNotARoleName;

impl FromStr for Role {
    type Err = StringIsNotARoleName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "admin" {
            Ok(Role::Admin)
        } else if s == "cashier" {
            Ok(Role::Cashier)
        } else {
            Err(StringIsNotARoleName)
        }
    }
}
