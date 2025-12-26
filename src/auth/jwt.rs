use super::types::Claims;
use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Clone)]
pub struct JwtSecret {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtSecret {
    pub fn new(secret: &str) -> Self {
        // Base64 encode the secret for added security
        let encoded_secret = general_purpose::STANDARD.encode(secret.as_bytes());

        Self {
            encoding_key: EncodingKey::from_secret(encoded_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(encoded_secret.as_bytes()),
        }
    }

    pub fn encoding_key(&self) -> &EncodingKey {
        &self.encoding_key
    }

    pub fn decoding_key(&self) -> &DecodingKey {
        &self.decoding_key
    }
}

pub fn encode_token(
    user_id: i64,
    username: &str,
    secret: &JwtSecret,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
    };

    encode(&Header::default(), &claims, secret.encoding_key())
}

pub fn decode_token(
    token: &str,
    secret: &JwtSecret,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(token, secret.decoding_key(), &Validation::default())?;
    Ok(token_data.claims)
}
