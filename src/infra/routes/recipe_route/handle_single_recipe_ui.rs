use crate::{
    app::{action::get_single_recipe, interface::Database},
    domain::entity::{Direction, Ingredient, Tag},
};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Html, Path},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;
use tera::{Context, Tera};

#[handler]
pub async fn handle_single_recipe_ui(
    recipe_id: Path<String>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // Init template engine
    let tera = Tera::new("src/web/pages/recipe/single/*.tera.html")
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Fetch single recipe
    let recipe = get_single_recipe(repo, &recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Inject recipes into template
    let mut context = Context::new();
    context.insert("title", &recipe.recipe_title);
    context.insert("favorite", &recipe.favorite);
    context.insert("servings", &recipe.servings);
    context.insert("ingredient_count", &recipe.ingredients.len());
    context.insert("direction_count", &recipe.directions.len());
    context.insert::<Vec<Ingredient>, &str>("ingredients", &recipe.ingredients);
    context.insert::<Vec<Direction>, &str>("directions", &recipe.directions);
    context.insert::<Vec<Tag>, &str>("tags", &recipe.tags);

    let rendered_html = tera
        .render("single_recipe.tera.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Html(rendered_html))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::{
            clients::db_client,
            helper::{get_random_recipe_id, get_test_session},
        },
        domain::constants::SESSION_COOKIE_KEY,
        infra::middleware::AuthGuard,
    };
    use poem::{get, middleware::AddData, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_single_recipe() {
        // build route
        let ep = Route::new()
            .at("/recipe/select/:id", get(handle_single_recipe_ui))
            .with(AddData::new(db_client().await))
            .with(AuthGuard);

        let test_client = TestClient::new(ep);

        // get a session token
        let session = get_test_session().await;

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
