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
            SELECT user_id, pantry_item_name, in_stock
            FROM pantry
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select pantry items for user")
    }
    async fn create_pantry_item(&self, name: &str, user_id: &str) -> Result<String> {
        sqlx::query(
            r#"
            INSERT INTO pantry (pantry_item_name, user_id)
            VALUES (?,?)
            "#,
        )
        .bind(&name)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .context("Failed to insert pantry item")?;

        Ok(name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::app::{clients::db_client, helper::get_test_user_id, interface::RecipeRepository};

    use super::*;

    #[tokio::test]
    async fn test_pantry_repo_creation() {
        let repo = db_client().await;
        let user_id = get_test_user_id().await;

        let recipe_id = repo
            .create_pantry_item("test_pantry_item", &user_id)
            .await
            .unwrap();

        let recipe = repo.select_recipe_by_id(&recipe_id).await.unwrap();
        assert_eq!(&recipe.recipe_title, "Oatmeal");
    }
}
