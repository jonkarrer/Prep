use poem::{
    handler,
    http::{HeaderMap, StatusCode},
    Error, Result,
};

use crate::{
    application::{decode_bearer_token, register_new_user},
    infra::database,
};

#[handler]
pub async fn handle_register_user(headers: &HeaderMap) -> Result<String> {
    let repo = database().await;
    match headers.get("Authorization") {
        Some(header_value) => {
            let bearer_token_string = header_value.to_str().map_err(|_| {
                Error::from_string(
                    "Invalid Authorization header",
                    poem::http::StatusCode::BAD_REQUEST,
                )
            })?;
            if !bearer_token_string.starts_with("Bearer ") {
                return Err(Error::from_string(
                    "Invalid Authorization header format",
                    StatusCode::BAD_REQUEST,
                ));
            }

            let encoded_token = &bearer_token_string["Bearer ".len()..];
            let basic_auth = decode_bearer_token(encoded_token)?;

            let user_id = register_new_user(&repo, basic_auth).await?;

            return Ok(user_id);
        }

        None => {
            return Err(Error::from_string(
                "Authorization header not found",
                StatusCode::BAD_REQUEST,
            ));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose, Engine};
    use poem::{post, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_register_user() {
        let app = Route::new().at("/register_user", post(handle_register_user));
        let test_client = TestClient::new(app);

        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password";

        let raw_token = format!("{}|{}", email, password);
        let encoded_token = general_purpose::STANDARD.encode(raw_token.as_bytes());
        let bearer_token = format!("Bearer {}", encoded_token);

        let mut resp = test_client
            .post("/register_user")
            .content_type("application/json")
            .header("Authorization", bearer_token)
            .send()
            .await;

        let id: String = resp.0.take_body().into_string().await.unwrap();
        dbg!(id);
        resp.assert_status_is_ok();
    }
}
