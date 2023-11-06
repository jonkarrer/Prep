use crate::{application::repository::RecipeRepository, domain::Recipe};
use poem::{
    handler,
    web::{Data, Json, Path},
    Result,
};
use std::sync::Arc;

#[handler]
pub async fn handle_get_recipe_by_id(
    recipe_id: Path<String>,
    repo: Data<&Arc<dyn RecipeRepository>>,
) -> Result<Json<Recipe>> {
    let recipe = repo.select_by_id(&recipe_id).await?;

    Ok(Json(recipe))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_get_recipe_by_id() {
        let app = Route::new().at("/recipe/:id", get(handle_get_recipe_by_id));
        let test_client = TestClient::new(app);

        let resp = test_client
            .get("/recipe/e947f008-2835-4e6f-9b80-edeb1ce096c9")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Gingerbread");
    }
}
