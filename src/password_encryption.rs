use sha2::{Sha256, Digest};

pub fn encrypt(password: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(password.to_owned() + &std::env::var("SALT").unwrap());
    hasher.finalize().to_vec()
}
