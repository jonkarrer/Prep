use crate::{
    app::interface::{Database, PantryRepository},
    domain::entity::PantryItem,
};
use anyhow::{Context, Result};
use sqlx::mysql::MySqlPool;

#[async_trait::async_trait]
impl PantryRepository for Database<MySqlPool> {
    async fn select_all_pantry_items(&self, user_id: &str) -> Result<Vec<PantryItem>> {
        sqlx::query_as(
            r#"
            SELECT user_id, ingredient_name, in_stock
            FROM pantry
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .context("Could Not Select Pantry Items For User")
    }
}
