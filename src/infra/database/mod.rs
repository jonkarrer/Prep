use crate::application::{
    helper::get_configuration,
    interface::{Database, RecipeRepository, UserRepository},
};
use crate::domain::{
    config::{DatabaseConfig, Settings},
    entity::{Direction, Ingredient, Recipe, RecipeArgs, Tag},
};
use anyhow::{Context, Result};
use serde_json::Value;
use sqlx::{mysql::MySqlPool, FromRow};

pub async fn db() -> Database<MySqlPool> {
    let Settings {
        database_config, ..
    } = get_configuration();

    Database::new(&database_config).await
}

impl Database<MySqlPool> {
    async fn new(config: &DatabaseConfig) -> Database<MySqlPool> {
        let addr = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user_name, config.password, config.host, config.port, config.db_name
        );
        let pool = MySqlPool::connect(addr.as_str())
            .await
            .expect("Failed connection with database");

        Database { pool }
    }
}

#[async_trait::async_trait]
impl RecipeRepository for Database<MySqlPool> {
    async fn create_recipe_from_args(&self, recipe: RecipeArgs, user_id: &str) -> Result<String> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .expect("transaction failed to start");

        let recipe_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO recipes (recipe_id, user_id, recipe_title, servings)
            VALUES (?,?,?,?)
            "#,
        )
        .bind(&recipe_id)
        .bind(user_id)
        .bind(recipe.title)
        .bind(recipe.servings)
        .execute(&self.pool)
        .await?;

        for ingredient in recipe.ingredients {
            sqlx::query(
                r#"
                INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit)
                VALUES (?,?,?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(ingredient.name)
            .bind(ingredient.amount)
            .bind(ingredient.unit)
            .execute(&mut *transaction)
            .await?;
        }

        for direction in recipe.directions {
            sqlx::query(
                r#"
                INSERT INTO directions (recipe_id, direction_details, step_order)
                VALUES (?,?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(direction.details)
            .bind(direction.step_order)
            .execute(&mut *transaction)
            .await?;
        }

        for tag_name in recipe.tags {
            sqlx::query(
                r#"
                INSERT INTO tags (recipe_id, tag_name)
                VALUES (?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(tag_name)
            .execute(&mut *transaction)
            .await?;
        }

        transaction
            .commit()
            .await
            .expect("Failed to commit transaction");

        Ok(recipe_id)
    }

    async fn select_by_id(&self, recipe_id: &str) -> Result<Recipe> {
        #[derive(FromRow)]
        struct RecipeDetails {
            pub recipe_id: String,
            pub recipe_title: String,
            pub servings: f32,
            pub favorite: bool,
        }
        let RecipeDetails {
            recipe_title,
            recipe_id,
            servings,
            favorite,
        } = sqlx::query_as(
            r#"
            SELECT recipe_id, recipe_title, servings, favorite
            FROM recipes
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to select recipe details by id")?;

        let ingredients: Vec<Ingredient> = sqlx::query_as(
            r#"
            SELECT ingredient_id, ingredient_name, amount, unit
            FROM ingredients
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select ingredients by id")?;

        let directions: Vec<Direction> = sqlx::query_as(
            r#"
            SELECT direction_id, step_order, direction_details
            FROM directions
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select directions by id")?;

        let tags: Vec<Tag> = sqlx::query_as(
            r#"
            SELECT tag_id, tag_name
            FROM tags
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select directions by id")?;

        let recipe = Recipe {
            recipe_id,
            recipe_title,
            servings,
            favorite,
            ingredients,
            directions,
            tags,
        };

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

#[async_trait::async_trait]
impl UserRepository for Database<MySqlPool> {
    async fn create_user(&self, email: &str, credentials_id: &str) -> Result<String> {
        let user_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO users (user_id, email, credential_id)
            VALUES (?,?,?)
            "#,
        )
        .bind(&user_id)
        .bind(email)
        .bind(credentials_id)
        .execute(&self.pool)
        .await
        .context("Failed to create user in database")?;

        Ok(user_id)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{application::helper::get_configuration, infra::test_helper::get_test_recipe_args};

    #[tokio::test]
    async fn test_recipe_repository() {
        let configs = get_configuration();
        let repo = Database::new(&configs.database_config).await;

        let recipe_args = get_test_recipe_args();
        let recipe_id = repo
            .create_recipe_from_args(recipe_args, "test_user_id")
            .await
            .unwrap();
        let recipe = repo.select_by_id(&recipe_id).await.unwrap();
        assert_eq!(&recipe.recipe_title, "Oatmeal");
    }
}
