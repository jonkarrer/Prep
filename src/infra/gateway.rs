use crate::application::repository::RecipeRepository;
use crate::configuration::DatabaseConfig;
use crate::domain::{Direction, Ingredient, Recipe, RecipeArgs, Tag};
use anyhow::{Context, Result};
use serde_json::Value;
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;

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
    async fn insert(&self, recipe: RecipeArgs, user_id: &str) -> Result<String> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .expect("transaction failed to start");

        // insert into recipe table
        let recipe_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            insert into recipes (recipe_id, user_id, recipe_title, servings)
            values (?,?,?,?)
            "#,
        )
        .bind(&recipe_id)
        .bind(user_id)
        .bind(recipe.title)
        .bind(recipe.servings)
        .execute(&self.pool)
        .await?;

        // insert into ingredients table
        for ingredient in recipe.ingredients {
            sqlx::query(
                r#"
                insert into ingredients (recipe_id, ingredient_name, amount, unit)
                values (?,?,?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(ingredient.name)
            .bind(ingredient.amount)
            .bind(ingredient.unit)
            .execute(&mut *transaction)
            .await?;
        }

        // insert into deirections table
        for direction in recipe.directions {
            sqlx::query(
                r#"
                insert into directions (recipe_id, direction_details, step_order)
                values (?,?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(direction.details)
            .bind(direction.step_order)
            .execute(&mut *transaction)
            .await?;
        }

        // insert into tags table
        for tag_name in recipe.tags {
            sqlx::query(
                r#"
                insert into tags (recipe_id, tag_name)
                values (?,?)
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

#[cfg(test)]
mod tests {
    use crate::{
        configuration::{get_configuration, Settings},
        domain::get_test_recipe_args,
    };

    use super::*;

    #[tokio::test]
    async fn test_recipe_repository() {
        let Settings { database, .. } = get_configuration();
        let repo = MySqlGateway::new(&database).await;

        let recipe_args = get_test_recipe_args();
        let recipe_id = repo.insert(recipe_args, "test_user_id").await.unwrap();
        let recipe = repo.select_by_id(&recipe_id).await.unwrap();
        assert_eq!(&recipe.recipe_title, "Oatmeal");
    }
}
