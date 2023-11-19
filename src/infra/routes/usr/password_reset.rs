use crate::{
    app::{
        interface::{Database, UserRepository},
        use_case::reset_password,
    },
    domain::entity::{PasswordResetToken, UpdatePasswordForm},
};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Form, Html},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[poem::handler]
pub async fn handle_password_reset_ui(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    let reset_token = PasswordResetToken::new();
    repo.insert_password_reset_token(&reset_token, &session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR))?;

    let user = repo.get_user_by_id(&session.user_id).await?;

    Ok(Html(format!(
        r#"
        <form hx-put="profile/password_reset">
        <label>
        Current Password
        <input type="password" name="current_password" />
        </lable>
        <label>
        New Password
        <input type="password" name="new_password" />
        </lable>
        <input type="hidden" name="reset_token" value={} />
        <input type="hidden" name="csrf_token" value={} />
        <input type="hidden" name="current_email" value={} />
        <button type="submit">Submit</button>
        </form>
        "#,
        reset_token.password_reset_token, session.csrf_token, user.email
    )))
}

#[poem::handler]
pub async fn handle_password_reset(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
    Form(req): Form<UpdatePasswordForm>,
) -> Result<String> {
    reset_password(session, repo, &req)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok("Password Has Been Updated".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_password_reset() {
        let path = "/usr/profile/password_reset";
        let app = Route::new().at(path, get(handle_password_reset));
        let test_client = TestClient::new(app);
        let resp = test_client.get(path).send().await;

        resp.assert_text("All Good Here. Keep Going").await;
    }
}
