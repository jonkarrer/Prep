use crate::app::interface::{Database, UserRepository};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Html},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[poem::handler]
pub async fn handle_user_profile_details(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    let user = repo
        .get_user_by_id(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Html(format!(
        r#"
        <div>Email: {}</div>
        <div>Profile Pic: {}</div>
        <div>Role: {}</div>
        "#,
        user.email, user.profile_pic_url, user.role
    )))
}
