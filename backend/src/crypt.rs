use argon2::{
    password_hash::{self, rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2, PasswordHasher,
};

pub fn compute_password_hash(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

// Verify password against PHC string.
pub fn verify_password(password: &str, password_hash: &str) -> Result<(), password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
