use crate::{app::use_case::login_user, domain::constants::SESSION_COOKIE_KEY};
use poem::{
    handler,
    http::StatusCode,
    web::{Form, Html},
    Error, IntoResponse, Request, Response, Result,
};

#[handler]
pub fn handle_login_ui(req: &Request) -> Result<impl IntoResponse> {
    match req.header("HX-Request") {
        Some(_) => Ok(Html(
            r#"
            <form action="/auth/login" method="POST" id="auth-form">
                <div>
                    <input 
                        type="email"
                        name="email"
                        placeholder="Email Address"
                        title="Enter an email address"
                    />
                </div>
                <div>
                    <input 
                        type="password"
                        name="password"
                        placeholder="Password"
                        title="Enter your password"
                    />
                </div>
                <button type="submit">Login</button>
            </form>
            "#,
        )),
        None => Err(Error::from_status(StatusCode::NOT_FOUND)),
    }
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[handler]
pub async fn handle_login(Form(req): Form<LoginRequest>) -> Result<Response> {
    let session = login_user(&req.email, &req.password).await.map_err(|_| {
        Error::from_string("Username or Password is incorrect", StatusCode::BAD_REQUEST)
    })?;

    let res = Response::builder()
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                SESSION_COOKIE_KEY, session.session_id
            ),
        )
        .header("Location", "/dash")
        .status(StatusCode::FOUND)
        .finish();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::app::helper::{TEST_USER_NAME, TEST_USER_PASSWORD};

    use super::*;
    use poem::{post, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_login() {
        // build route
        let path = "/usr/login";
        let ep = Route::new().at(path, post(handle_login));
        let test_client = TestClient::new(ep);

        // set test creds, this matches the seeder
        let form_data = [("email", TEST_USER_NAME), ("password", TEST_USER_PASSWORD)];

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
