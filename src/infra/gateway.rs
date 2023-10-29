use crate::configuration::DatabaseConfig;
use crate::domain::RecipeRecord;
use crate::{application::RecipeRepository, domain::Recipe};
use anyhow::{Context, Result};
use serde_json::Value;
use sqlx::mysql::MySqlPool;

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let addr = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user_name, config.password, config.host, config.port, config.db_name
        );
        let pool = MySqlPool::connect(addr.as_str())
            .await
            .expect("Failed connection with MySql database");

        Self { pool }
    }
}

#[async_trait::async_trait]
impl RecipeRepository for MySqlGateway {
    type RecipeId = String;
    async fn insert(&self, recipe: Recipe, user_id: &str) -> Result<Self::RecipeId> {
        let blob: Value = serde_json::to_value(recipe)?;
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO recipes (id, user_id, recipe)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(blob)
        .execute(&self.pool)
        .await
        .context("Failed to insert recipe")?;

        Ok(id)
    }

    async fn select_by_id(&self, recipe_id: &str) -> Result<Recipe> {
        let recipe_record: RecipeRecord = sqlx::query_as(
            r#"
            SELECT id, user_id, recipe 
            FROM recipes
            WHERE id = ?
            "#,
        )
        .bind(recipe_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to get recipe by id")?;

        let recipe: Recipe = serde_json::from_value(recipe_record.recipe)?;

        Ok(recipe)
    }

    async fn update(&self, new_recipe: Recipe, recipe_id: &str) -> Result<()> {
        let blob: Value = serde_json::to_value(new_recipe)?;

        sqlx::query(
            r#"
            UPDATE recipes
            SET recipe = ?
            WHERE id = ?;
            "#,
        )
        .bind(blob)
        .bind(recipe_id)
        .execute(&self.pool)
        .await
        .context("Failed to update recipe")?;

        Ok(())
    }

    async fn delete(&self, recipe_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM recipes 
            WHERE id = ?
            "#,
        )
        .bind(recipe_id)
        .execute(&self.pool)
        .await
        .context("Failed to delete recipe")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_surreal_gateway() {
        let recipe = Recipe {
            ingredients: vec![
                "1 1/2 pounds ground beef".to_string(),
                "1/2 cup breadcrumbs".to_string(),
            ],
            instructions: vec![
                "Preheat the oven to 350°F (175°C).".to_string(),
                "In a large bowl, combine all the ingredients.".to_string(),
            ],
            title: "Classic Meatloaf".to_string(),
        };

        let db_config = DatabaseConfig {
            host: "localhost".to_string(),
            password: "my-secret-pw".to_string(),
            db_name: "mysql".to_string(),
            user_name: "root".to_string(),
            port: 3306,
        };

        let repo = MySqlGateway::new(&db_config).await;

        // Test insert
        let id = repo.insert(recipe, "jon@gmail").await.unwrap();

        // Test select by recipe id
        let recipe = repo.select_by_id(id.as_str()).await.unwrap();
        assert_eq!(recipe.title, "Classic Meatloaf");
    }
}
