use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use std::env;

pub fn hash_password(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(res) => Some(res.to_string()),
        Err(_) => None,
    }
}

pub fn is_valid_argon2(hashed: &str) -> bool {
    PasswordHash::new(&hashed).is_ok()
}

pub fn is_authorized(password: &str) -> bool {
    let actual_hashed = env::var("ADMIN_AUTH").unwrap();
    match PasswordHash::new(&actual_hashed) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}
