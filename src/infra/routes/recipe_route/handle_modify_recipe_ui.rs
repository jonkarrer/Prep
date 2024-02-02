use crate::{
    app::{
        action::get_single_recipe,
        interface::{Database, RecipeRepository},
    },
    domain::entity::{Direction, Ingredient},
};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Html, Path},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;
use tera::{Context, Tera};

#[handler]
pub async fn handle_modify_recipe_ui(
    recipe_id: Path<String>,
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // Init template engine
    let tera = Tera::new("src/web/pages/recipe/modify/*.tera.html")
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Fetch single recipe
    let recipe = get_single_recipe(repo, &recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Fetch tags
    let tags = repo
        .select_tags_for_user(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    let selected_tags: Vec<String> = recipe.tags.into_iter().map(|item| item.tag_name).collect();
    let all_tag_options: Vec<(String, bool)> = tags
        .into_iter()
        .map(|item| {
            let is_checked = selected_tags.contains(&item.tag_name);
            (item.tag_name, is_checked)
        })
        .collect();

    // Inject recipes into template
    let mut context = Context::new();
    context.insert("title", &recipe.recipe_title);
    context.insert("recipe_id", &recipe.recipe_id);
    context.insert("favorite", &recipe.favorite);
    context.insert("servings", &recipe.servings);
    context.insert("ingredient_count", &recipe.ingredients.len());
    context.insert("direction_count", &recipe.directions.len());
    context.insert::<Vec<Ingredient>, &str>("ingredients", &recipe.ingredients);
    context.insert::<Vec<Direction>, &str>("directions", &recipe.directions);
    context.insert::<Vec<(String, bool)>, &str>("all_tags", &all_tag_options);
    context.insert::<Vec<String>, &str>("selected_tags", &selected_tags);

    let rendered_html = tera
        .render("modify_recipe.tera.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Html(rendered_html))
}
