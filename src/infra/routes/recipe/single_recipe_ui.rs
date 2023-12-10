use crate::{
    app::interface::{Database, RecipeRepository},
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
    let tera = Tera::new("src/web/pages/recipe/*.html.tera")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;

    // Fetch single recipe
    let recipe = repo
        .select_by_recipe_id(&recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Inject recipes into template
    let mut context = Context::new();
    context.insert("title", &recipe.recipe_title);
    context.insert("favorite", &recipe.favorite);
    context.insert::<Vec<Ingredient>, &str>("ingredients", &recipe.ingredients);
    context.insert::<Vec<Direction>, &str>("directions", &recipe.directions);
    context.insert::<Vec<Tag>, &str>("tags", &recipe.tags);

    let rendered_html = tera
        .render("single_recipe.html.tera", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Html(rendered_html))
}
