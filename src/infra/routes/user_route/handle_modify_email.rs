use crate::{
    app::{action::update_user_email, interface::Database},
    domain::entity::UpdateEmailForm,
};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};
use sqlx::MySqlPool;

#[poem::handler]
pub async fn handle_modify_email(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
    Form(req): Form<UpdateEmailForm>,
) -> Result<Response> {
    update_user_email(session, repo, &req)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR))?;

    let res = Response::builder()
        .header("Location", "/usr/account")
        .status(StatusCode::FOUND)
        .finish();

    Ok(res)
}
