use crate::{
    app::interface::{Database, PantryRepository},
    domain::entity::PantryItem,
};
use anyhow::{Context, Result};
use sqlx::mysql::MySqlPool;

impl PantryRepository for Database<MySqlPool> {
    async fn select_all_pantry_items(&self, user_id: &str) -> Result<Vec<PantryItem>> {
        sqlx::query_as(
            r#"
            SELECT user_id, item_name, in_stock
            FROM pantry
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select pantry items for user")
    }

    async fn create_pantry_item(&self, name: &str, user_id: &str) -> Result<PantryItem> {
        sqlx::query(
            r#"
            INSERT INTO pantry (item_name, user_id)
            VALUES (?,?)
            "#,
        )
        .bind(&name)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .context("Failed to insert pantry item")?;

        Ok(PantryItem {
            item_name: name.to_string(),
            user_id: user_id.to_string(),
            in_stock: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{clients::db_client, helper::get_test_user_id};

    #[tokio::test]
    async fn test_pantry_repo_creation() {
        let repo = db_client().await;
        let user_id = get_test_user_id().await;

        let test_name = "test_pantry_item";
        let pantry_item = repo.create_pantry_item(test_name, &user_id).await.unwrap();

        assert_eq!(&pantry_item.item_name, test_name);
    }

    #[tokio::test]
    async fn test_pantry_repo_select_all() {
        let repo = db_client().await;
        let user_id = get_test_user_id().await;

        let all = repo.select_all_pantry_items(&user_id).await.unwrap();

        assert!(all.len() != 0);
    }
}
