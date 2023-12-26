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
    async fn create_pantry_item(&self, name: String, user_id: &str) -> Result<String> {
        // TODO change column name to pantry_item_name
        sqlx::query(
            r#"
            INSERT INTO pantry (ingredient_name, user_id)
            VALUES (?,?)
            "#,
        )
        .bind(&name)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .context("Failed to insert pantry item")?;

        Ok(name)
    }
}
