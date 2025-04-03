use crate::PgPool;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use base64::{Engine as _, engine::general_purpose};
use password_hash::PasswordHash;
use password_hash::rand_core::OsRng;
use rand::Rng;
use sha2::{Digest, Sha256};

#[derive(sqlx::FromRow, sqlx::Type, Debug)]
pub struct CreateApiKey {
    pub sha: String,
    pub api_key_hash: String,
    pub username: String,
    pub label: String,
}

impl CreateApiKey {
    pub async fn insert(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO api_keys (sha, user_id, api_key_hash, label)
        VALUES ($1, $2, $3, $4)
        "#,
            self.sha,
            self.username,
            self.api_key_hash,
            self.label,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct ApiKey {
    pub sha: String,
    pub hashed_key: String,
    pub key: String,
}

pub fn generate_api_key() -> ApiKey {
    let key_bytes: [u8; 32] = rand::rng().random();
    let key = general_purpose::STANDARD.encode(key_bytes);
    let sha = key_sha(&key);

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_key = argon2
        .hash_password(key.as_bytes(), &salt)
        .expect("Failed to hash API key")
        .to_string();

    ApiKey {
        sha,
        key,
        hashed_key,
    }
}

pub fn key_sha(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let sha = format!("{:x}", hasher.finalize());
    sha
}

pub fn verify_api_key(api_key: &str, hashed_key: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_key).expect("Invalid hash format");
    Argon2::default()
        .verify_password(api_key.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_api_key() {
        let api_key = generate_api_key();
        assert!(verify_api_key(&api_key.key, &api_key.hashed_key));
        assert_eq!(api_key.sha, key_sha(&api_key.key));
    }
}
