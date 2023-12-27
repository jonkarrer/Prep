mod handle_all_recipes_ui;
mod handle_create_recipe;
mod handle_single_recipe_ui;

use crate::{
    app::configs::StaticPath,
    infra::middleware::{AuthGuard, AuthGuardImpl},
};
use handle_all_recipes_ui::*;
use handle_create_recipe::*;
use handle_single_recipe_ui::*;
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};

pub fn use_recipe_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_recipes_ui))
        .at("/select/:id", get(handle_single_recipe_ui))
        .at(
            "/create",
            get(StaticFileEndpoint::new(
                StaticPath::from("/pages/recipe/create/create_recipe.html").0,
            ))
            .post(handle_create_recipe),
        )
        .with(AuthGuard)
}

#[cfg(test)]
mod tests {
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    use super::*;
    use crate::app::clients::db_client;
    use crate::app::helper::{get_random_recipe_id, get_test_recipe_args, get_test_session};
    use crate::domain::constants::SESSION_COOKIE_KEY;
    use crate::domain::entity::RecipeDetails;
    use crate::infra::middleware::AuthGuard;

    #[tokio::test]
    async fn test_route_create_recipe() {
        // build route
        let path = "/recipe/create";
        let ep = Route::new()
            .at(path, post(handle_create_recipe))
            .with(AddData::new(db_client().await))
            .with(AuthGuard);

        let test_client = TestClient::new(ep);

        // get a session token
        let session = get_test_session().await.unwrap();

        // create fake recipe
        let test_recipe = get_test_recipe_args();
        let payload = serde_json::to_string(&test_recipe).unwrap();

        // run test
        let resp = test_client
            .post(path)
            .body(payload)
            .header(
                "Cookie",
                format!("{}={}", SESSION_COOKIE_KEY, session.session_id),
            )
            .header("X-CSRF-Token", session.csrf_token)
            .content_type("application/json")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: RecipeDetails = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Oatmeal");
    }

    #[tokio::test]
    async fn test_route_select_recipe() {
        // build route
        let ep = Route::new()
            .at("/select/:id", get(handle_single_recipe_ui))
            .with(AddData::new(db_client().await))
            .with(AuthGuard);

        let test_client = TestClient::new(ep);

        // get a session token
        let session = get_test_session().await.unwrap();

        // get random recipe id to use
        let recipe_id = get_random_recipe_id().await.unwrap();
        let path = format!("/recipe/select/{}", recipe_id);

        // run test
        let resp = test_client
            .get(path)
            .header(
                "Cookie",
                format!("{}={}", SESSION_COOKIE_KEY, session.session_id),
            )
            .send()
            .await;

        resp.assert_status_is_ok();

        let content_type = resp
            .0
            .headers()
            .get("Content-Type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(content_type.starts_with("text/html"));
    }
}
