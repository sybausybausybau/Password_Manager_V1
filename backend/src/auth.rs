use jsonwebtoken::TokenData;
pub use serde::{Deserialize, Serialize};
pub use jsonwebtoken::{encode, decode, DecodingKey, Validation, Algorithm, Header, EncodingKey};
pub use chrono::Utc;
use crate::error::ServerError;
use axum_extra::extract::{cookie::Cookie, cookie::SameSite::Strict, CookieJar};

const JWT_TIME_TO_LIVE : i64 = 900;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    iat: String, // issued at
    exp: usize, // seconde
}

pub async fn create_token(user_id: String, secret: String) -> Result<String, ServerError>{
    let now = Utc::now().timestamp();
    let my_claims = Claims {sub: user_id, iat: now.to_string(), exp: (now + JWT_TIME_TO_LIVE) as usize};
    Ok(encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref()))?)
}

pub async fn decode_token(token: String, secret: String) -> Result<TokenData<Claims>, ServerError>{
    Ok(decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256))?)
}

pub async fn create_cookie(token: String) -> Result<CookieJar, ServerError> {
    let jar = CookieJar::new().add(
            Cookie::build(("token", token))
            .http_only(true)
            .secure(true)
            .path("/")
            .same_site(Strict)
            .build()
    );
    Ok(jar)
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