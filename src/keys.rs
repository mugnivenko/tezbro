use std::borrow::Borrow;

use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct Key(pub Hmac<Sha256>);

impl Key {
    pub fn inner(&self) -> Hmac<Sha256> {
        self.0
    }
}

impl<C: Borrow<[u8]>> From<C> for Key {
    fn from(contents: C) -> Self {
        Key(Hmac::new_from_slice(contents.borrow()).unwrap())
    }
}

pub struct Keys {
    admin: Key,
    cashier: Key,
}

impl Keys {
    pub fn new(admin: Key, cashier: Key) -> Self {
        Self { admin, cashier }
    }

    pub fn admin(&self) -> &Key {
        &self.admin
    }

    pub fn cashier(&self) -> &Key {
        &self.cashier
    }
}
