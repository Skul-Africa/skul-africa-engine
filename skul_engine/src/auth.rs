use anyhow::{Result, anyhow};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    pub school_id: String,
    pub exp: usize,
}

/// Get the authenticated school ID from a JWT token
pub fn get_authenticated_school_id(token: &str) -> Result<String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secretkey".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|e| anyhow!("Invalid token: {}", e))?;

    Ok(token_data.claims.school_id)
}
