use crate::{
    application::interface::{Database, UserRepository},
    infra::{authentication::auth, service::decode_bearer_token},
};
use poem::{
    handler,
    http::{HeaderMap, StatusCode},
    web::Data,
    Error, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_register_user(
    headers: &HeaderMap,
    repo: Data<&Database<MySqlPool>>,
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

            let mut auth = auth().await;

            let credentials_id = auth
                .register(&basic_auth.email, &basic_auth.password)
                .await
                .map_err(|e| {
                    Error::from_string(format!("{e}"), poem::http::StatusCode::CONFLICT)
                })?;

            let user_id = repo
                .create_user(&basic_auth.email, credentials_id.as_str())
                .await
                .map_err(|e| {
                    Error::from_string(format!("{e}"), poem::http::StatusCode::BAD_GATEWAY)
                })?;

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
    use crate::infra::test_helper::init_test_client_with_db;
    use base64::{engine::general_purpose, Engine};
    use poem::post;

    #[tokio::test]
    async fn test_route_register_user() {
        let path = "/usr/register";
        let test_client = init_test_client_with_db(path, post(handle_register_user)).await;

        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password";

        let raw_token = format!("{}|{}", email, password);
        let encoded_token = general_purpose::STANDARD.encode(raw_token.as_bytes());
        let bearer_token = format!("Bearer {}", encoded_token);

        let resp = test_client
            .post(path)
            .header("Authorization", bearer_token)
            .send()
            .await;

        resp.assert_status_is_ok();

        // TODO select by id in db to confirm registration
        // let id: String = resp.0.take_body().into_string().await.unwrap();
    }
}
