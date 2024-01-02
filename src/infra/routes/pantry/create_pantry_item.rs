use crate::{
    app::{action::create_pantry_item, interface::Database},
    domain::entity::PantryItem,
};
use brize_auth::entity::Session;
use poem::{
    handler,
    web::{Data, Json},
    Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_create_pantry_item(
    item_name: String,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<Json<PantryItem>> {
    let pantry_item = create_pantry_item(repo, &item_name, &session.user_id).await?;

    Ok(Json(pantry_item))
}
