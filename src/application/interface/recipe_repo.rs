use crate::domain::{NewRecipe, Recipe};
use anyhow::Result;

#[async_trait::async_trait]
pub trait RecipeRepository {
    async fn insert(&self, recipe: NewRecipe, user_id: &str) -> Result<String>;
    async fn select_by_id(&self, id: &str) -> Result<Recipe>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn update(&self, new_recipe: Recipe, id: &str) -> Result<()>;
}
