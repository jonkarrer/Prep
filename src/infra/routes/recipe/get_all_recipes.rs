use crate::app::interface::{Database, RecipeRepository};
use brize_auth::entity::Session;
use poem::{
    handler,
    web::{Data, Html},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_get_all_recipes_ui(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // TODO add htmx protection header
    let recipes = repo
        .select_all_recipe_metadata_for_user(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::NOT_FOUND))?;

    let mut html_string = String::from("<ul>");
    for detail in recipes {
        let list_item = format!(
            "<li><a href=\"/recipe/select/{}\">{}</a></li>",
            detail.recipe_id, detail.recipe_title
        );
        html_string.push_str(&list_item)
    }
    html_string.push_str("</ul>");

    Ok(Html(html_string))
}
