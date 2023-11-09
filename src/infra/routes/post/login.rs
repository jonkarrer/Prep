use crate::infra::{authentication::auth, service::BasicAuthParams};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    csrf_token: String,
    email: String,
    password: String,
}

#[handler]
pub async fn handle_login(Form(req): Form<LoginRequest>) -> Result<Response> {
    let mut auth = auth().await;
    let session_token: String = auth
        .login(&req.email, &req.password)
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
        .header("X-CSRF-Token", req.csrf_token)
        .header("Location", "/dashboard")
        .status(StatusCode::SEE_OTHER)
        .body("Login Successful");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::middleware::BasicAuth;
    use base64::{engine::general_purpose, Engine};
    use poem::{post, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_login() {
        // build route
        let path = "/usr/login";
        let ep = Route::new().at(path, post(handle_login));
        let test_client = TestClient::new(ep);

        // set test creds, this matches the seeder
        let email = "seed_user@gmail.com";
        let password = "seeder_password";

        // encode creds
        let raw_token = format!("{}|{}", email, password);
        let encoded_token = general_purpose::STANDARD.encode(raw_token.as_bytes());
        let bearer_token = format!("Bearer {}", encoded_token);

        // run test
        let resp = test_client
            .post(path)
            .header("Authorization", bearer_token)
            .send()
            .await;

        // assert results
        resp.assert_status_is_ok();
        resp.assert_text("Login Successful").await;

        // TODO select from session table with the returned id
    }
}
