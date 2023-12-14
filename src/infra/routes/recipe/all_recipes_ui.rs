use crate::{
    app::interface::{Database, RecipeRepository},
    domain::entity::RecipeDetails,
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
pub async fn handle_all_recipes_ui(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // Init template engine
    let tera = Tera::new("src/web/pages/recipe/all/*.html")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;

    // Fetch all recipes
    let recipes = repo
        .select_all_recipe_metadata_for_user(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Inject recipes into template
    let mut context = Context::new();
    context.insert::<Vec<RecipeDetails>, &str>("recipes", &recipes);

    let rendered_html = tera
        .render("all_recipes.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Serve template
    Ok(Html(rendered_html))
}
