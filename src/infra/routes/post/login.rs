use crate::infra::{authentication::auth, service::BasicAuthParams};
use poem::{handler, http::StatusCode, web::Data, Error, Response, Result};

#[handler]
pub async fn handle_login(Data(basic_auth): Data<&BasicAuthParams>) -> Result<Response> {
    let mut auth = auth().await;
    let session_token: String = auth
        .login(&basic_auth.email, &basic_auth.password)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::CONFLICT))?;
    let csrf_token = "my_csrf_token";

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
        .body("Login Successful");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::middleware::BasicAuth;
    use base64::{engine::general_purpose, Engine};
    use poem::{get, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_login() {
        // build route
        let path = "/usr/login";
        let ep = Route::new().at(path, get(handle_login)).with(BasicAuth);
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
            .get(path)
            .header("Authorization", bearer_token)
            .send()
            .await;

        // assert results
        resp.assert_status_is_ok();
        resp.assert_text("Login Successful").await;

        // TODO select from session table with the returned id
    }
}
