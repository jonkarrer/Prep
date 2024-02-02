use crate::{
    app::{
        action::{modify_recipe, validate_recipe_args},
        interface::Database,
    },
    domain::entity::RecipeArgs,
};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
    Error, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_modify_recipe(
    Json(recipe_args): Json<RecipeArgs>,
    recipe_id: Path<String>,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<()> {
    if !validate_recipe_args(&recipe_args) {
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }

    modify_recipe(repo, &recipe_id, recipe_args, &session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::BAD_GATEWAY))?;

    Ok(())
}
