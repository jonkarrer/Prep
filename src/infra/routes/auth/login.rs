use crate::{
    application::interface::UserRepository,
    infra::{
        authentication::{auth_client, session_client},
        database::db,
    },
};
use brize_auth::config::Expiry;
use poem::{handler, http::StatusCode, web::Form, Error, Response, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[handler]
pub async fn handle_login(Form(req): Form<LoginRequest>) -> Result<Response> {
    // Pass auth check
    auth_client()
        .await
        .verify_credentials(&req.email, &req.password)
        .await
        .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

    // Get user_id
    let user = db()
        .await
        .get_user_by_email(&req.email)
        .await
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    // Start session
    let session = session_client()
        .await
        .start_session(&user.user_id, Expiry::Month(1))
        .await
        .map_err(|_| Error::from_status(StatusCode::BAD_GATEWAY))?;

    let mut response = Response::builder()
        .header(
            "Set-Cookie",
            format!(
                "session_id={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                session.session_id
            ),
        )
        .header(
            "Set-Cookie",
            format!(
                "csrf_token={}; Path=/; Secure; SameSite=Strict",
                session.csrf_token
            ),
        )
        .header("Location", "/usr/dashboard")
        .status(StatusCode::FOUND)
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
        let form_data = [("email", email), ("password", password)];

        // run test
        let resp = test_client
            .post(path)
            .content_type("application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await;

        // assert results
        resp.assert_text("Login Successful").await;

        // TODO select from session table with the returned id
    }
}
