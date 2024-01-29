use crate::app::{action::delete_recipe, interface::Database};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form, Path},
    Response, Result,
};
use sqlx::MySqlPool;

#[derive(serde::Deserialize)]
pub struct DeleteRecipeForm {
    csrf_token: String,
}

#[handler]
pub async fn handle_delete_recipe(
    recipe_id: Path<String>,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
    Form(req): Form<DeleteRecipeForm>,
) -> Result<Response> {
    let mut resp = Response::builder();
    match delete_recipe(repo, session, &recipe_id, &req.csrf_token).await {
        Ok(_) => Ok(resp
            .header("Location", "/recipe/all")
            .status(StatusCode::PERMANENT_REDIRECT)
            .finish()),
        Err(_) => Ok(resp.status(StatusCode::UNAUTHORIZED).finish()),
    }
}
