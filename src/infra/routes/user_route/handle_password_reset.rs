use crate::{
    app::{action::reset_password, interface::Database},
    domain::entity::UpdatePasswordForm,
};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Form},
    Error, Result,
};
use sqlx::MySqlPool;

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
