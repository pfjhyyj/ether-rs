use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

use crate::env;


pub fn generate_jwt_token<T>(claims: &T) -> Result<String, Error>
where T: Serialize {
  let jwt_secret = env::get_env::<String>("JWT_SECRET");
  let key = EncodingKey::from_secret(jwt_secret.as_bytes());
  let token = encode(&Header::default(), &claims, &key);

  token
}

pub fn verify_jwt_token<T>(token: &str) -> Result<T, Error>
where T: DeserializeOwned {
  let jwt_secret = env::get_env::<String>("JWT_SECRET");
  let key = DecodingKey::from_secret(jwt_secret.as_bytes());
  let token_data = decode::<T>(&token, &key, &Validation::default())
    .map(|data| data.claims);

  token_data
}