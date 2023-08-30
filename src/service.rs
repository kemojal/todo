pub fn hash_password(password: &str) -> String {
    // Create a new Argon2 configuration
    let config = Config::default();

    // Hash the password and return the encoded hash
    hash_encoded(password.as_bytes(), &config).expect("Failed to hash password")
}