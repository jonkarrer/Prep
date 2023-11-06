use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};

pub struct BasicAuth {
    pub email: String,
    pub password: String,
}
impl BasicAuth {
    pub fn from_token_string(token: String) -> Self {
        let split: Vec<&str> = token.split("|").collect();
        let email = split[0];
        let password = split[1];

        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
pub fn decode_bearer_token(encoded_token: &str) -> Result<BasicAuth> {
    let decoded_bytes = general_purpose::STANDARD.decode(encoded_token).unwrap();
    let token = String::from_utf8(decoded_bytes).unwrap();
    Ok(BasicAuth::from_token_string(token))
}
