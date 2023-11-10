use crate::infra::authentication::auth;
use poem::{handler, http::StatusCode, web::Form, Error, Response, Result};
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
    let (session_token, csrf_token) = auth
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
        .header("X-CSRF-Token", csrf_token)
        .header("Location", "/dashboard")
        .status(StatusCode::SEE_OTHER)
        .body("Login Successful");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{post, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_login() {
        // build route
        let path = "/usr/login";
        let ep = Route::new().at(path, post(handle_login));
        let test_client = TestClient::new(ep);

        // set test creds, this matches the seeder
        let email = "seed_user@gmail.com";
        let password = "seeder_password";
        let csrf_token = "my_csrf_tokn";
        let form_data = [
            ("email", email),
            ("password", password),
            ("csrf_token", csrf_token),
        ];

        // run test
        let resp = test_client
            .post(path)
            .content_type("application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await;

        // assert results
        // resp.assert_status_is_ok();
        resp.assert_text("Login Successful").await;

        // TODO select from session table with the returned id
    }
}
