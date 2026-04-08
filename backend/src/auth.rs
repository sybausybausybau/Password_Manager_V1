use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   company: String
}


#[cfg(tests)]
pub mod tests {
    use super::*;

    
    let my_claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned()
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();

}