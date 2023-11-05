use crate::application::store_user_recipe;
use crate::configuration::{get_configuration, Settings};
use crate::domain::{Recipe, RecipeArgs};
use crate::infra::MySqlGateway;
use poem::web::Json;
use poem::{handler, Result};

#[handler]
pub async fn create_recipe(Json(recipe): Json<RecipeArgs>) -> Result<Json<Recipe>> {
    let Settings { database, .. } = get_configuration();
    let repo = MySqlGateway::new(&database).await;
    let recipe = store_user_recipe(&repo, recipe, "route_user_test").await?;

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
        let app = Route::new().at("/create_recipe", post(create_recipe));
        let test_client = TestClient::new(app);

        let payload = serde_json::to_string(&test_recipe).unwrap();
        let resp = test_client
            .post("/create_recipe")
            .body(payload)
            .content_type("application/json")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();

        assert_eq!(json.recipe_title, "Oatmeal");
    }
}
