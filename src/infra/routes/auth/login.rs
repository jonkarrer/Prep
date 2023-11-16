use crate::{
    app::case::{start_session_for_user, verify_user_credentials},
    domain::entity::{CSRF_COOKIE_KEY, SESSION_COOKIE_KEY},
};
use poem::{handler, http::StatusCode, web::Form, Error, Response, Result};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[handler]
pub async fn handle_login(Form(req): Form<LoginRequest>) -> Result<Response> {
    let user_id = verify_user_credentials(&req.email, &req.password)
        .await
        .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

    let session = start_session_for_user(&user_id.0)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let res = Response::builder()
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                SESSION_COOKIE_KEY, session.session_id
            ),
        )
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path=/; Secure; SameSite=Strict",
                CSRF_COOKIE_KEY, session.csrf_token
            ),
        )
        .header("Location", "/usr/dashboard")
        .status(StatusCode::FOUND)
        .finish();

    Ok(res)
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
        resp.assert_status(StatusCode::FOUND);

        // TODO select from session table with the returned id
    }
}
