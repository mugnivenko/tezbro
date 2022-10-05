use std::borrow::Borrow;
use bcrypt::bcrypt;

pub fn encrypt(password: impl Borrow<[u8]>) -> impl Borrow<[u8]> {
    const COST: u32 = 12;
    let mut salt: [u8; 16] = Default::default();
    salt.copy_from_slice(std::env::var("SALT").unwrap().as_bytes());
    bcrypt(COST, salt, password.borrow())
}
