use crate::{
    app::interface::{Database, RecipeRepository},
    domain::entity::{Direction, Ingredient, Recipe, RecipeArgs, RecipeDetails, Tag},
};
use anyhow::{Context, Result};
use serde_json::Value;
use sqlx::mysql::MySqlPool;

#[async_trait::async_trait]
impl RecipeRepository for Database<MySqlPool> {
    async fn create_recipe_from_args(
        &self,
        recipe_args: RecipeArgs,
        user_id: &str,
    ) -> Result<String> {
        let recipe_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO recipes (recipe_id, user_id, recipe_title, servings)
            VALUES (?,?,?,?)
            "#,
        )
        .bind(&recipe_id)
        .bind(user_id)
        .bind(recipe_args.title)
        .bind(recipe_args.servings)
        .execute(&self.pool)
        .await
        .context("Failed to insert into recipes")?;

        let mut transaction = self
            .pool
            .begin()
            .await
            .expect("Failed to start transaction");

        for ingredient in recipe_args.ingredients {
            sqlx::query(
                r#"
                INSERT INTO ingredients (recipe_id, user_id, ingredient_name, amount, unit)
                VALUES (?,?,?,?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(user_id)
            .bind(ingredient.name)
            .bind(ingredient.amount)
            .bind(ingredient.unit)
            .execute(&mut *transaction)
            .await
            .context("Failed to insert into ingredients")?;
        }

        for direction in recipe_args.directions {
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
            .await
            .context("Failed to insert into directions")?;
        }

        for tag_name in recipe_args.tags {
            sqlx::query(
                r#"
                INSERT INTO tags (recipe_id, tag_name)
                VALUES (?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(tag_name)
            .execute(&mut *transaction)
            .await
            .context("Failed to insert into tags")?;
        }

        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;

        Ok(recipe_id)
    }

    async fn select_recipe_by_id(&self, recipe_id: &str) -> Result<Recipe> {
        let RecipeDetails {
            recipe_title,
            recipe_id,
            servings,
            favorite,
        } = self.select_recipe_details_by_id(&recipe_id).await?;

        let ingredients = self.select_ingredients_for_recipe(&recipe_id).await?;
        let directions = self.select_directions_for_recipe(&recipe_id).await?;
        let tags = self.select_tags_for_recipe(&recipe_id).await?;

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

    async fn select_recipe_by_title(&self, recipe_title: &str, user_id: &str) -> Result<Recipe> {
        let RecipeDetails {
            recipe_title,
            recipe_id,
            servings,
            favorite,
        } = self
            .select_recipe_details_by_title(recipe_title, user_id)
            .await?;

        let ingredients = self.select_ingredients_for_recipe(&recipe_id).await?;
        let directions = self.select_directions_for_recipe(&recipe_id).await?;
        let tags = self.select_tags_for_recipe(&recipe_id).await?;

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

    async fn select_ingredients_for_recipe(&self, recipe_id: &str) -> Result<Vec<Ingredient>> {
        sqlx::query_as(
            r#"
            SELECT ingredient_id, ingredient_name, amount, unit
            FROM ingredients
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select ingredients by id")
    }

    async fn select_directions_for_recipe(&self, recipe_id: &str) -> Result<Vec<Direction>> {
        sqlx::query_as(
            r#"
            SELECT direction_id, step_order, direction_details
            FROM directions
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select directions by id")
    }

    async fn select_tags_for_recipe(&self, recipe_id: &str) -> Result<Vec<Tag>> {
        sqlx::query_as(
            r#"
            SELECT tag_id, tag_name
            FROM tags
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select directions by id")
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

    async fn select_all_recipe_details_for_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<RecipeDetails>> {
        sqlx::query_as(
            r#"
            SELECT recipe_id, recipe_title, servings, favorite
            FROM recipes
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select recipe details by user id")
    }

    async fn select_recipe_details_by_id(&self, recipe_id: &str) -> Result<RecipeDetails> {
        sqlx::query_as(
            r#"
            SELECT recipe_id, recipe_title, servings, favorite
            FROM recipes
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to select recipe details by id")
    }

    async fn select_recipe_details_by_title(
        &self,
        recipe_title: &str,
        user_id: &str,
    ) -> Result<RecipeDetails> {
        sqlx::query_as(
            r#"
            SELECT recipe_id, recipe_title, servings, favorite
            FROM recipes
            WHERE recipe_title = ? AND user_id = ? 
            "#,
        )
        .bind(&recipe_title)
        .bind(&user_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to select recipe details by title")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{configs::DbConfig, helper::get_test_recipe_args};

    #[tokio::test]
    async fn test_recipe_repo_creation() {
        let config = DbConfig::default();
        let repo = Database::new(&config).await;

        let recipe_args = get_test_recipe_args();
        let recipe_id = repo
            .create_recipe_from_args(recipe_args, "test_user_id")
            .await
            .unwrap();

        let recipe = repo.select_recipe_by_id(&recipe_id).await.unwrap();
        assert_eq!(&recipe.recipe_title, "Oatmeal");
    }
}
