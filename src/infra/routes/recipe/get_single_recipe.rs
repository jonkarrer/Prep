use crate::app::interface::{Database, RecipeRepository};
use poem::{
    handler,
    web::{Data, Html, Path},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_get_single_recipe_ui(
    recipe_id: Path<String>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    let recipe = repo
        .select_by_recipe_id(&recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::NOT_FOUND))?;

    let mut html_string = String::from("<div>");
    html_string.push_str(format!("<h2>{}</h2>", recipe.recipe_title).as_str());
    html_string.push_str(format!("<div>Favorite:{}</div>", recipe.favorite).as_str());

    html_string.push_str("<ul><h3>Ingredients</h3>");
    for ing in recipe.ingredients {
        let list_item = format!(
            "<li><span>{} {} {}</span></li>",
            ing.amount, ing.unit, ing.ingredient_name
        );
        html_string.push_str(&list_item)
    }
    html_string.push_str("</ul>");

    html_string.push_str("<ul><h3>Directions</h3>");
    for dir in recipe.directions {
        let list_item = format!(
            "<li><span>{}. {}</span></li>",
            dir.step_order, dir.direction_details
        );
        html_string.push_str(&list_item)
    }
    html_string.push_str("</ul>");

    html_string.push_str("<ul><h3>Tags</h3>");
    for tag in recipe.tags {
        let list_item = format!("<li><span>{}</span></li>", tag.tag_name);
        html_string.push_str(&list_item)
    }
    html_string.push_str("</ul>");

    html_string.push_str("</div>");
    Ok(Html(html_string))
}
