use axum::http::{HeaderMap};
use jsonwebtoken::TokenData;
use log::error;
pub use serde::{Deserialize, Serialize};
pub use jsonwebtoken::{encode, decode, DecodingKey, Validation, Algorithm, Header, EncodingKey};
pub use chrono::Utc;
use crate::error::ServerError;
use axum_extra::extract::{cookie::Cookie, cookie::SameSite::Strict};

const JWT_TIME_TO_LIVE : i64 = 900;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    iat: String, // issued at
    exp: usize, // seconde
}

pub async fn create_token(user_id: String, secret: &str) -> Result<String, ServerError>{
    let now = Utc::now().timestamp();
    let my_claims = Claims {sub: user_id, iat: now.to_string(), exp: (now + JWT_TIME_TO_LIVE) as usize};
    Ok(encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_bytes()))?)
}

pub async fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, ServerError>{
    Ok(decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256))?)
}

pub async fn create_cookie(token: String) -> Result<Cookie<'static>, ServerError> {
    let cookie = 
        Cookie::build(("token", token))
        .http_only(true)
        .secure(true)
        .path("/")
        .same_site(Strict)
        .build();
    
    Ok(cookie)
}

pub async fn get_token(headers : HeaderMap, jwt_secret : &str) -> Result<TokenData<Claims>, ServerError> { // Extract and decode JWT token from HTTP headers
    let extracted_token = headers
        .get("Authorization")
        .ok_or_else(|| {
            error!("Authorization header is missing.");
            ServerError::UnknownError("Authorization header is missing.".to_string())
        })?
        .to_str()
        .map_err(|err| ServerError::UnknownError(err.to_string()))?;
    
    let decoded_token = decode_token(extracted_token, jwt_secret)
        .await
        .map_err(|err| ServerError::UnknownError(err.to_string()))?;

    Ok(decoded_token)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn create_token_test() {
        let my_claims = Claims {
            sub: "b@b.com".to_owned(),
            iat: "ACME".to_owned(),
            exp: (Utc::now().timestamp() + 3600) as usize, // expires in 1 hour
        };

        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
            .unwrap();
        println!("{}", token)

    }

    #[test]
    fn decode_token_test() {
        let my_claims = Claims {
            sub: "b@b.com".to_owned(),
            iat: "ACME".to_owned(),
            exp: (Utc::now().timestamp() + 3600) as usize,
        };

        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
            .unwrap();

        let token_message = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256));
        println!("{:?}", token_message);
        //assert_eq!();
    }
}