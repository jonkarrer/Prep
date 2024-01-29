use crate::{
    app::{
        action::{create_recipe, validate_recipe_args},
        interface::Database,
    },
    domain::entity::{RecipeArgs, RecipeDetails},
};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    Error, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_modify_recipe(
    Json(recipe_args): Json<RecipeArgs>,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<Json<RecipeDetails>> {
    if !validate_recipe_args(&recipe_args) {
        return Err(Error::from_status(StatusCode::BAD_REQUEST));
    }

    let recipe = create_recipe(repo, recipe_args, &session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::BAD_GATEWAY))?;

    Ok(Json(recipe))
}
