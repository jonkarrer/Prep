use crate::{
    app::{
        interface::Database,
        use_case::{register_new_user, start_session_for_user},
    },
    domain::constants::SESSION_COOKIE_KEY,
};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form, Html},
    Error, IntoResponse, Request, Response, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[handler]
pub fn handle_register_ui(req: &Request) -> Result<impl IntoResponse> {
    match req.header("HX-Request") {
        Some(_) => Ok(Html(
            r#"
            <form action="/auth/register" method="POST">
                <div>
                    <input type="text" name="email" placeholder="Email Address" />
                </div>
                <div>
                    <input type="password" name="password" placeholder="Password" />
                </div>
                <div>
                    <input
                        type="confirm_password"
                        name="confirm_password"
                        placeholder="Confirm Password"
                    />
                </div>
                <button type="submit">Register</button>
            </form>
            "#,
        )),
        None => Err(Error::from_status(StatusCode::NOT_FOUND)),
    }
}

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
    let user_id = register_new_user(&req.email, &req.password, repo)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

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
        .header("Location", "/dash")
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
