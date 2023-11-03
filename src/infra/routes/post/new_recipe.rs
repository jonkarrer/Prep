use crate::domain::NewRecipe;
use poem::web::Json;
use poem::{handler, Result};

#[handler]
pub async fn new_recipe(Json(recipe): Json<NewRecipe>) -> Result<String> {
    println!("recipe: {:?}", &recipe);
    Ok(recipe.title)
}

#[cfg(test)]
mod tests {
    use crate::domain::get_test_recipe;

    use super::*;
    use poem::{post, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_new_recipe() {
        let test_recipe = get_test_recipe();
        let app = Route::new().at("/new_recipe", post(new_recipe));
        let test_client = TestClient::new(app);

        let payload = serde_json::to_string(&test_recipe).unwrap();
        let resp = test_client
            .post("/new_recipe")
            .body(payload)
            .content_type("application/json")
            .send()
            .await;

        resp.assert_text(test_recipe.title).await;
    }
}
