use sha2::{Sha256, Digest};

pub fn encrypt_password(password: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.finalize().to_vec()
}
