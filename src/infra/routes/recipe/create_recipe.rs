use crate::{
    app::interface::{Database, RecipeRepository},
    domain::entity::{Recipe, RecipeArgs},
};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    Error, Result,
};
use sqlx::MySqlPool;

fn validate_recipe(recipe: &RecipeArgs) -> bool {
    let title = recipe.title.as_str();

    if title.len() == 0 {
        return false;
    }

    for dir in recipe.directions.iter() {
        if dir.details.len() == 0 {
            return false;
        }
    }

    for ing in recipe.ingredients.iter() {
        let ingredient_name = ing.name.as_str();

        if ingredient_name.len() == 0 {
            return false;
        }
    }

    true
}

#[handler]
pub async fn handle_create_recipe(
    Json(recipe): Json<RecipeArgs>,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<Json<Recipe>> {
    // TODO run validation on input before insertion

    if !validate_recipe(&recipe) {
        return Err(Error::from_string(
            "Recipe missing some things",
            StatusCode::BAD_REQUEST,
        ));
    }

    let recipe_id = repo
        .create_recipe_from_args(recipe, &session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::BAD_GATEWAY))?;

    let recipe = repo
        .select_recipe_by_id(&recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::BAD_GATEWAY))?;

    Ok(Json(recipe))
}

#[cfg(test)]
mod tests {
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    use super::*;
    use crate::app::clients::db_client;
    use crate::app::helper::{get_test_recipe_args, get_test_session};
    use crate::domain::constants::SESSION_COOKIE_KEY;
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

        let json: Recipe = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Oatmeal");
    }
}
