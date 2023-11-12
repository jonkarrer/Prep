use crate::infra::authentication::session_client;
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogoutForm {
    csrf_token: String,
}

#[handler]
pub async fn handle_logout(
    Data(session): Data<&Session>,
    Form(req): Form<LogoutForm>,
) -> Result<Response> {
    if session.match_csrf_token(&req.csrf_token) {
        // Destroy session, thus logging the user out
        session_client()
            .await
            .destory_session(&session.session_id)
            .await
            .map_err(|_| Error::from_status(StatusCode::BAD_GATEWAY))?;

        let mut response = Response::builder().header(
            "Set-Cookie",
            "session_id=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; Secure; SameSite=Strict"
        ).header(
            "Set-Cookie",
            "csrf_token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Secure; SameSite=Strict",
        )
        .header("Location", "/auth/login")
        .status(StatusCode::SEE_OTHER)
        .body("Logout Successful");

        Ok(response)
    } else {
        Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized"))
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::middleware::AuthGuard;

    use super::*;
    use poem::{post, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_login() {
        // build route
        let path = "/usr/login";
        let ep = Route::new().at(path, post(handle_logout)).with(AuthGuard);
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
