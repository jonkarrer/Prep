use crate::{
    app::interface::{Database, RecipeRepository},
    domain::entity::Tag,
};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Html},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;
use tera::{Context, Tera};

#[handler]
pub async fn handle_create_recipe_ui(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // Init template engine
    let tera = Tera::new("src/web/pages/recipe/create/*.tera.html")
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Fetch tags
    let tags = repo
        .select_tags_for_user(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Inject recipes into template
    let mut context = Context::new();
    context.insert::<Vec<Tag>, &str>("tags", &tags);

    let rendered_html = tera
        .render("create_recipe.tera.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Html(rendered_html))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::{clients::db_client, helper::get_test_session},
        domain::constants::SESSION_COOKIE_KEY,
        infra::middleware::AuthGuard,
    };
    use poem::{get, middleware::AddData, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_single_recipe() {
        // build route
        let path = "/recipe/create";
        let ep = Route::new()
            .at(path, get(handle_create_recipe_ui))
            .with(AddData::new(db_client().await))
            .with(AuthGuard);

        let test_client = TestClient::new(ep);

        // get a session token
        let session = get_test_session().await;

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
