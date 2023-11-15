use crate::{
    app::{
        case::{register_new_user, start_session_for_user},
        interface::Database,
    },
    domain::entity::{CSRF_COOKIE_KEY, SESSION_COOKIE_KEY},
};
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
    // Register user
    let user_id = register_new_user(&req.email, &req.password, repo)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Start session
    let session = start_session_for_user(&user_id.0)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let mut response = Response::builder()
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
        .body("Registration Successful");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::app::clients::db_client;

    use super::*;
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_register_user() {
        // build route
        let path = "/usr/register";
        let ep = Route::new()
            .at(path, post(handle_register))
            .with(AddData::new(db_client().await));
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
