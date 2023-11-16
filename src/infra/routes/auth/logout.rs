use crate::{
    app::case::logout_user,
    domain::entity::{CSRF_COOKIE_KEY, SESSION_COOKIE_KEY},
};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form},
    Response, Result,
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
    let mut resp = Response::builder();
    match logout_user(&session, &req.csrf_token).await {
        Ok(_) => {
            Ok(
                resp
                .header(
                    "Set-Cookie",
                    format!("{}=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; Secure; SameSite=Strict", 
                    SESSION_COOKIE_KEY
                    )
                )
                .header(
                    "Set-Cookie",
                    format!("{}=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Secure; SameSite=Strict", 
                    CSRF_COOKIE_KEY
                    ),
                )
                .header("Location", "/auth/login")
                .status(StatusCode::FOUND)
                .finish()
            )
        }
        Err(_) => Ok(resp.status(StatusCode::UNAUTHORIZED).finish()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{app::clients::session_client, infra::middleware::AuthGuard};

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
