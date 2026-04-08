use jsonwebtoken::TokenData;
pub use serde::{Deserialize, Serialize};
pub use jsonwebtoken::{encode, decode, DecodingKey, Validation, Algorithm, Header, EncodingKey};
pub use chrono::Utc;
use crate::error::ServerError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub iat: String, // issued at
    pub exp: usize,
}

pub async fn create_token(user_id: String, secret: String) -> Result<String, ServerError>{
    let now = Utc::now().timestamp();
    let my_claims = Claims {sub: user_id, iat: now.to_string(), exp: (now + 90) as usize};
    Ok(encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref()))?)
}

pub async fn decode_token(token: String, secret: String) -> Result<TokenData<Claims>, ServerError>{
    Ok(decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256))?)
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