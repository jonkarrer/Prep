use crate::app::interface::{Database, UserRepository};
use brize_auth::entity::Session;
use chrono::{Duration, Utc};
use poem::{
    http::StatusCode,
    web::{Data, Html},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[poem::handler]
pub async fn handle_password_reset(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    let reset_token = uuid::Uuid::new_v4().to_string();
    let reset_duration = Duration::hours(1);
    let expiration = Utc::now() + reset_duration;

    repo.insert_reset_password_details(&reset_token, &expiration, &session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Html(format!(
        r#"
        <input type="password" name="new_password" />
        <div>secret_token: {}</div>
        <div>expiration: {}</div>
        "#,
        reset_token, expiration
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_password_reset() {
        let path = "/auth/password_reset";
        let app = Route::new().at(path, get(handle_password_reset));
        let test_client = TestClient::new(app);
        let resp = test_client.get(path).send().await;

        resp.assert_text("All Good Here. Keep Going").await;
    }
}
