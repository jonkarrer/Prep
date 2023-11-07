use crate::{
    application::interface::{Database, RecipeRepository},
    domain::entity::Recipe,
};
use poem::{
    handler,
    web::{Data, Json, Path},
    Error, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_get_recipe_by_id(
    recipe_id: Path<String>,
    repo: Data<&Database<MySqlPool>>,
) -> Result<Json<Recipe>> {
    let recipe = repo
        .select_by_id(&recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::BAD_GATEWAY))?;

    Ok(Json(recipe))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::test_helper::init_test_client_with_db;
    use poem::get;

    #[tokio::test]
    async fn test_route_get_recipe_by_id() {
        let test_client =
            init_test_client_with_db("/recipe/:id", get(handle_get_recipe_by_id)).await;

        let resp = test_client
            .get("/recipe/30457f09-012c-4668-803e-fb84c04ff1e4")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Gingerbread");
    }
}
