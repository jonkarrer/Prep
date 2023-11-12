use crate::{
    application::interface::{Database, UserRepository},
    infra::authentication::{auth_client, session_client},
};
use brize_auth::config::Expiry;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[handler]
pub async fn handle_register(
    Form(req): Form<RegisterRequest>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<Response> {
    // Register creds
    let creds_id = auth_client()
        .await
        .register(&req.email, &req.password)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::CONFLICT))?;

    // Register user
    let user_id = repo
        .create_user(&req.email, creds_id.as_str())
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::BAD_GATEWAY))?;

    // Start session
    let session = session_client()
        .await
        .start_session(&user_id, Expiry::Month(1))
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
        .status(StatusCode::SEE_OTHER)
        .body("Registration Successful");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::infra::database::db;

    use super::*;
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_register_user() {
        // build route
        let path = "/usr/register";
        let ep = Route::new()
            .at(path, post(handle_register))
            .with(AddData::new(db().await));
        let test_client = TestClient::new(ep);

        // create random user creds
        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password";
        let form_data = [("email", email), ("password", password)];

        // run test
        let resp = test_client
            .post(path)
            .content_type("application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await;

        // assert result
        resp.assert_text("Registration Successful").await;

        // TODO select by id in db to confirm registration
        // let id: String = resp.0.take_body().into_string().await.unwrap();
    }
}
