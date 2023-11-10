use crate::{
    application::interface::{Database, UserRepository},
    infra::{authentication::auth, service::BasicAuthParams},
};
use poem::{handler, http::StatusCode, web::Data, Error, Response, Result};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_register(
    Data(repo): Data<&Database<MySqlPool>>,
    Data(basic_auth): Data<&BasicAuthParams>,
) -> Result<Response> {
    let mut auth = auth().await;

    let credentials_id = auth
        .register(&basic_auth.email, &basic_auth.password)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::CONFLICT))?;

    repo.create_user(&basic_auth.email, credentials_id.as_str())
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::BAD_GATEWAY))?;

    let (session_token, csrf_token) = auth
        .login(&basic_auth.email, &basic_auth.password)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::CONFLICT))?;

    let mut response = Response::builder()
        .header(
            "Set-Cookie",
            format!(
                "session_id={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                session_token
            ),
        )
        .header("X-CSRF-Token", csrf_token)
        .status(StatusCode::OK)
        .body("Registration Successful");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::infra::{db, middleware::BasicAuth};

    use super::*;
    use base64::{engine::general_purpose, Engine};
    use poem::{get, middleware::AddData, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_register_user() {
        // build route
        let path = "/usr/register";
        let ep = Route::new()
            .at(path, get(handle_register))
            .with(BasicAuth)
            .with(AddData::new(db().await));
        let test_client = TestClient::new(ep);

        // create random user creds
        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password";

        // create bearer token
        let raw_token = format!("{}|{}", email, password);
        let encoded_token = general_purpose::STANDARD.encode(raw_token.as_bytes());
        let bearer_token = format!("Bearer {}", encoded_token);

        // run test
        let resp = test_client
            .get(path)
            .header("Authorization", bearer_token)
            .send()
            .await;

        // assert result
        resp.assert_status_is_ok();
        resp.assert_text("Registration Successful").await;

        // TODO select by id in db to confirm registration
        // let id: String = resp.0.take_body().into_string().await.unwrap();
    }
}
