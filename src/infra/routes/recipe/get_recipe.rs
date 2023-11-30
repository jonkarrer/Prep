use crate::{
    app::interface::{Database, RecipeRepository},
    domain::entity::Recipe,
};
use poem::{
    handler,
    web::{Data, Html, Json, Path},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_get_recipe_ui(
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::app::helper::TEST_USER_NAME;
//     use crate::app::interface::UserRepository;
//     use crate::app::{clients::db_client, helper::get_test_session};
//     use crate::domain::constants::SESSION_COOKIE_KEY;
//     use crate::infra::middleware::AuthGuard;
//     use poem::{get, middleware::AddData, test::TestClient, EndpointExt, Route};

//     #[tokio::test]
//     async fn test_route_get_recipe() {
//         // build route
//         let path = "/recipe/select/:id";
//         let ep = Route::new()
//             .at(path, get(handle_get_recipe))
//             .with(AddData::new(db_client().await))
//             .with(AuthGuard);

//         let test_client = TestClient::new(ep);

//         // get a session token
//         let session = get_test_session().await.unwrap();

//         // get the seeded gingerbread recipe
//         let repo = db_client().await;
//         let user = repo.get_user_by_email(TEST_USER_NAME).await.unwrap();
//         let test_recipe = repo
//             .select_by_recipe_title("Gingerbread", &user.user_id)
//             .await
//             .unwrap();

//         // use the gingerbread recipe id as a test
//         let query = format!("/recipe/select/{}", test_recipe.recipe_id);
//         let resp = test_client
//             .get(query)
//             .header(
//                 "Cookie",
//                 format!("{}={}", SESSION_COOKIE_KEY, session.session_id),
//             )
//             .send()
//             .await;

//         // resp.assert_status_is_ok();

//         let json: Recipe = resp.json().await.value().deserialize();
//         assert_eq!(json.recipe_title, "Gingerbread");
//     }
// }
