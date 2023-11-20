use crate::{
    app::{interface::Database, use_case::update_user_email},
    domain::entity::UpdateEmailForm,
};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Form, Html},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[poem::handler]
pub async fn handle_update_email_ui(Data(session): Data<&Session>) -> Result<impl IntoResponse> {
    Ok(Html(format!(
        r#"
        <form hx-put="profile/update_email">
        <input type="hidden" name="csrf_token" value={} />
        <input type="text" name="new_email" />
        <button type="submit">Submit</button>
        </form>
        "#,
        session.csrf_token
    )))
}

#[poem::handler]
pub async fn handle_update_email(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
    Form(req): Form<UpdateEmailForm>,
) -> Result<String> {
    update_user_email(session, repo, &req)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok("Updated Email".to_string())
}
