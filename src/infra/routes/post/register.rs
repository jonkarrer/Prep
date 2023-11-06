use std::sync::Arc;

use crate::{
    application::repository::UserRepository,
    infra::{authentication::init_auth_client, service::decode_bearer_token},
};
use poem::{
    handler,
    http::{HeaderMap, StatusCode},
    web::Data,
    Error, Result,
};

#[handler]
pub async fn handle_register_user(
    headers: &HeaderMap,
    repo: Data<&Arc<dyn UserRepository>>,
) -> Result<String> {
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

            let mut auth = init_auth_client().await?;

            let credentials_id = auth
                .register(&basic_auth.email, &basic_auth.password)
                .await?;

            let user_id = repo
                .create(&basic_auth.email, credentials_id.as_str())
                .await?;

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
