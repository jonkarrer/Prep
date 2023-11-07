use poem::{
    handler,
    http::{HeaderMap, StatusCode},
    Error, Result,
};

use crate::infra::{authentication::auth, service::decode_bearer_token};

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

            let mut auth = auth().await;
            let session_token: String = auth.login(&basic_auth.email, &basic_auth.password).await?;
            Ok(session_token)
        }

        None => Err(Error::from_string(
            "Authorization header not found",
            StatusCode::BAD_REQUEST,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::test_helper::init_test_client_with_db;
    use base64::{engine::general_purpose, Engine};
    use poem::post;

    #[tokio::test]
    async fn test_route_login() {
        let path = "/usr/login";
        let test_client = init_test_client_with_db(path, post(handle_login)).await;

        let email = "seed_user@gmail.com";
        let password = "seeder_password";

        let raw_token = format!("{}|{}", email, password);
        let encoded_token = general_purpose::STANDARD.encode(raw_token.as_bytes());
        let bearer_token = format!("Bearer {}", encoded_token);

        let resp = test_client
            .post(path)
            .header("Authorization", bearer_token)
            .send()
            .await;

        resp.assert_status_is_ok();

        // TODO select from session table with the returned id
    }
}
