use crate::{
    app::interface::{Database, PantryRepository},
    domain::entity::PantryItem,
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
pub async fn handle_all_pantry_items_ui(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // Init template engine
    let tera = Tera::new("src/web/pages/pantry/all/*.tera.html")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;

    // Fetch all pantry items
    let pantry_items = repo
        .select_all_pantry_items(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Inject recipes into template
    let mut context = Context::new();
    context.insert::<Vec<PantryItem>, &str>("pantry_items", &pantry_items);

    let rendered_html = tera
        .render("all_pantry_items.tera.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Serve template
    Ok(Html(rendered_html))
}
