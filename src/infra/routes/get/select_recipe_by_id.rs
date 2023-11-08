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
pub async fn handle_select_recipe_by_id(
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
    use crate::infra::{db, middleware::AuthGuard, test_helper::get_test_session_token};
    use poem::{
        get,
        middleware::{AddData, CookieJarManager},
        test::TestClient,
        EndpointExt, Route,
    };

    #[tokio::test]
    async fn test_route_select_recipe_by_id() {
        // build route
        let path = "/recipe/select/:id";
        let ep = Route::new()
            .at(path, get(handle_select_recipe_by_id))
            .with(AddData::new(db().await))
            .with(AuthGuard)
            .with(CookieJarManager::new());
        let test_client = TestClient::new(ep);

        // get a session token
        let session_token = get_test_session_token().await;

        // ! will fail on a new seed. Id will be stale
        // TODO create a test helper that gets the id for the Gingerbread recipe
        let resp = test_client
            .get("/recipe/select/a11aaa36-0114-4bdf-8e40-5c266705b7ad")
            .header("Cookie", format!("session_id={}", session_token))
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Gingerbread");
    }
}
