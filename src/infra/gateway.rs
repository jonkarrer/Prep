use crate::configuration::DatabaseConfig;
use crate::domain::{NewRecipe, RecipeRecord};
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
            .expect("Failed connection with database");

        Self { pool }
    }
}

#[async_trait::async_trait]
impl RecipeRepository for MySqlGateway {
    async fn insert(&self, recipe: NewRecipe, user_id: &str) -> Result<String> {
        let recipe_id = uuid::Uuid::new_v4().to_string();

        sqlx::query(
            r#"
        INSERT INTO recipes (recipe_id, user_id, title, servings)
        VALUES (?,?,?,?)
        "#,
        )
        .bind(&recipe_id)
        .bind(user_id)
        .bind(recipe.title)
        .bind(recipe.servings)
        .execute(&self.pool)
        .await?;

        Ok(recipe_id)
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
