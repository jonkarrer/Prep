use crate::{
    application::repository::RecipeRepository,
    domain::{Recipe, RecipeArgs},
};
use poem::{
    handler,
    web::{Data, Json},
    Result,
};
use std::sync::Arc;

#[handler]
pub async fn handle_create_recipe(
    Json(recipe): Json<RecipeArgs>,
    repo: Data<&Arc<dyn RecipeRepository>>,
) -> Result<Json<Recipe>> {
    let recipe_id = repo.create_from_args(recipe, "user_test_id").await?;
    let recipe = repo.select_by_id(&recipe_id).await?;

    Ok(Json(recipe))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::get_test_recipe_args;
    use poem::{post, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_create_recipe() {
        let test_recipe = get_test_recipe_args();
        let app = Route::new().at("/new_recipe", post(handle_create_recipe));
        let test_client = TestClient::new(app);

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
