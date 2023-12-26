use brize_auth::entity::Session;
use poem::{handler, web::Data, Result};
use sqlx::MySqlPool;

use crate::app::interface::{Database, PantryRepository};

#[handler]
pub async fn handle_create_pantry_item(
    pantry_item_name: String,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<String> {
    println!("{pantry_item_name}");

    let item_name = repo
        .create_pantry_item(pantry_item_name, &session.user_id)
        .await?;

    Ok(item_name)
}
