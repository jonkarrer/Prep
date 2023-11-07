use crate::infra::{authentication::auth, service::BasicAuthParams};
use poem::{handler, http::StatusCode, web::Data, Error, Result};

#[handler]
pub async fn handle_login(Data(basic_auth): Data<&BasicAuthParams>) -> Result<String> {
    let mut auth = auth().await;
    let session_token: String = auth
        .login(&basic_auth.email, &basic_auth.password)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::CONFLICT))?;

    // HTTP/1.1 200 OK
    // Set-Cookie: sessionid=abc123; Path=/; Secure; HttpOnly; SameSite=Strict
    // Content-Type: application/json

    // {
    //     "success": "Logged in successfully"
    // }

    Ok(session_token)
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
        let ep = Route::new().at(path, post(handle_login)).with(BasicAuth);
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

        // TODO select from session table with the returned id
    }
}
