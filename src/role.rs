pub enum Role {
    Admin,
    Cashier,
}

impl Role {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Cashier => "cashier",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(Self::Admin),
            "cashier" => Some(Self::Cashier),
            _ => None,
        }
    }
}
