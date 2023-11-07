use crate::{
    application::interface::{Database, RecipeRepository},
    domain::entity::{Recipe, RecipeArgs},
};
use poem::{
    handler,
    web::{Data, Json},
    Error, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_create_recipe(
    Json(recipe): Json<RecipeArgs>,
    repo: Data<&Database<MySqlPool>>,
) -> Result<Json<Recipe>> {
    let recipe_id = repo
        .create_from_args(recipe, "user_test_id")
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::BAD_GATEWAY))?;

    let recipe = repo
        .select_by_id(&recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::BAD_GATEWAY))?;

    Ok(Json(recipe))
}

#[cfg(test)]
mod tests {
    use poem::post;

    use super::*;
    use crate::infra::test_helper::{get_test_recipe_args, init_test_client_with_db};

    #[tokio::test]
    async fn test_route_create_recipe() {
        let test_recipe = get_test_recipe_args();
        let test_client = init_test_client_with_db("/new_recipe", post(handle_create_recipe)).await;

        let payload = serde_json::to_string(&test_recipe).unwrap();
        let resp = test_client
            .post("/new_recipe")
            .body(payload)
            .content_type("application/json")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();

        assert_eq!(json.recipe_title, "Oatmeal");
    }
}
