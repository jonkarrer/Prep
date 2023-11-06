use poem::{
    handler,
    http::{HeaderMap, StatusCode},
    Error, Result,
};

use crate::infra::{authentication::init_auth_client, service::decode_bearer_token};

#[handler]
pub async fn handle_login(headers: &HeaderMap) -> Result<String> {
    match headers.get("Authorization") {
        Some(header_value) => {
            let bearer_token_string = header_value.to_str().map_err(|_| {
                Error::from_string("Invalid Authorization header", StatusCode::BAD_REQUEST)
            })?;

            if !bearer_token_string.starts_with("Bearer ") {
                return Err(Error::from_string(
                    "Invalid Authorization header format",
                    StatusCode::BAD_REQUEST,
                ));
            }

            let encoded_token = &bearer_token_string["Bearer ".len()..];
            let basic_auth = decode_bearer_token(encoded_token)?;

            let mut auth = init_auth_client().await?;
            let session_token: String = auth.login(&basic_auth.email, &basic_auth.password).await?;
            Ok(session_token)
        }

        None => Err(Error::from_string(
            "Authorization header not found",
            StatusCode::BAD_REQUEST,
        )),
    }
}
