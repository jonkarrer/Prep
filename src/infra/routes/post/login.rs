use crate::application::{decode_bearer_token, login_user};
use poem::{
    handler,
    http::{HeaderMap, StatusCode},
    Error, Result,
};

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

            let session_id = login_user(basic_auth).await?;

            Ok(session_id)
        }

        None => Err(Error::from_string(
            "Authorization header not found",
            StatusCode::BAD_REQUEST,
        )),
    }
}
