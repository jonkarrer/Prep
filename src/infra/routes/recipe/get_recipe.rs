use crate::{
    app::interface::{Database, RecipeRepository},
    domain::entity::Recipe,
};
use poem::{
    handler,
    web::{Data, Json, Path},
    Error, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_get_recipe(
    recipe_id: Path<String>,
    Data(repo): Data<&Database<MySqlPool>>,
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
    use crate::app::{clients::db_client, helper::get_test_session};
    use crate::domain::constants::SESSION_COOKIE_KEY;
    use crate::infra::middleware::AuthGuard;
    use poem::{get, middleware::AddData, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_select_recipe_by_id() {
        // build route
        let path = "/recipe/select/:id";
        let ep = Route::new()
            .at(path, get(handle_get_recipe))
            .with(AddData::new(db_client().await))
            .with(AuthGuard);

        let test_client = TestClient::new(ep);

        // get a session token
        let session = get_test_session().await.unwrap();

        // ! will fail on a new seed. Id will be stale
        // TODO create a test helper that gets the id for the Gingerbread recipe
        let resp = test_client
            .get("/recipe/select/f0458ac2-7b93-4866-971b-2a2d7f457c13")
            .header(
                "Cookie",
                format!("{}={}", SESSION_COOKIE_KEY, session.session_id),
            )
            .send()
            .await;

        // resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Gingerbread");
    }
}
