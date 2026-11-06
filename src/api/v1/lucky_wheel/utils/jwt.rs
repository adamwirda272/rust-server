use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    AccessToken,
    SessionToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub token_type: TokenType,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(token_type: TokenType, subject: &str) -> Self {
        let exp_duration = match token_type {
            TokenType::AccessToken => Duration::days(30),
            TokenType::SessionToken => Duration::seconds(30),
        };
        Self {
            token_type,
            sub: subject.to_string(),
            exp: (Utc::now() + exp_duration).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        }
    }
}

pub fn create_token(token_type: TokenType, subject: &str, secret_key: impl AsRef<[u8]>) -> String {
    let claims = Claims::new(token_type, subject);
    let encoding_key = &EncodingKey::from_secret(secret_key.as_ref());

    return encode(&Header::default(), &claims, encoding_key).unwrap();
}

pub fn verify_token(token: &str, secret_key: impl AsRef<[u8]>) -> Result<Claims, Error> {
    let decoding_key = &DecodingKey::from_secret(secret_key.as_ref());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "iat"]);

    let token_data = decode::<Claims>(&token, decoding_key, &validation)?;

    let current_timestamp = jsonwebtoken::get_current_timestamp() as usize;
    if token_data.claims.iat > current_timestamp {
        return Err(jsonwebtoken::errors::ErrorKind::ImmatureSignature.into());
    };

    Ok(token_data.claims)
}
