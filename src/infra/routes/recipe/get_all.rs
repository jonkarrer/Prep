use crate::{
    app::interface::{Database, RecipeRepository},
    domain::entity::Recipe,
};
use poem::{
    handler,
    web::{Data, Html, Path},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_get_all_recipes_ui(
    user_id: Path<String>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    let recipes = repo
        .select_all_recipe_metadata_for_user(&user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::NOT_FOUND))?;

    let mut html_string = String::from("<ul>");
    for detail in recipes {
        let list_item = format!("<li>{}</li>", detail.recipe_title);
        html_string.push_str(&list_item)
    }
    html_string.push_str("</ul>");

    Ok(Html(html_string))
}
