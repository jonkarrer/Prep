use crate::{
    app::{action::delete_account, interface::Database},
    domain::entity::DeleteAccountForm,
};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};
use sqlx::MySqlPool;

#[poem::handler]
pub async fn handle_delete_account(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
    Form(req): Form<DeleteAccountForm>,
) -> Result<Response> {
    delete_account(repo, &req, session)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let res = Response::builder()
        .header("Location", "/")
        .status(StatusCode::FOUND)
        .finish();

    Ok(res)
}
