use crate::infra::clients::session_client;
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};

#[derive(serde::Deserialize)]
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
        .status(StatusCode::FOUND).finish();

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
    async fn test_route_logout() {
        // build route
        let path = "/usr/lougout";
        let ep = Route::new().at(path, post(handle_logout)).with(AuthGuard);
        let test_client = TestClient::new(ep);
        let mut s = session_client().await;

        // set test creds
        let user_id = uuid::Uuid::new_v4().to_string();
        let session = s
            .start_session(&user_id, brize_auth::config::Expiry::Day(1))
            .await
            .unwrap();

        // run test
        let resp = test_client
            .post(path)
            .header("Cookie", format!("session_id={}", &session.session_id))
            .content_type("application/x-www-form-urlencoded")
            .form(&[("csrf_token", session.csrf_token)])
            .send()
            .await;

        // assert results
        resp.assert_status(StatusCode::FOUND);
        assert!(s.get_session(&session.session_id).await.is_err())
    }
}
