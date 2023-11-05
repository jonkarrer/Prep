use crate::domain::{Recipe, RecipeArgs};
use anyhow::Result;

#[async_trait::async_trait]
pub trait RecipeRepository {
    async fn create_from_args(&self, recipe: RecipeArgs, user_id: &str) -> Result<String>;
    async fn select_by_id(&self, id: &str) -> Result<Recipe>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn update(&self, new_recipe: Recipe, id: &str) -> Result<()>;
}
