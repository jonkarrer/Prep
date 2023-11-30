use crate::domain::entity::{Recipe, RecipeArgs, RecipeDetails, RecipeMetadata};
use anyhow::Result;

#[async_trait::async_trait]
pub trait RecipeRepository: Send + Sync {
    async fn create_recipe_from_args(&self, recipe: RecipeArgs, user_id: &str) -> Result<String>;
    async fn select_by_recipe_id(&self, recipe_id: &str) -> Result<Recipe>;
    async fn select_by_recipe_title(&self, recipe_title: &str, user_id: &str) -> Result<Recipe>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn update(&self, new_recipe: Recipe, id: &str) -> Result<()>;
    async fn select_all_recipe_metadata_for_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<RecipeDetails>>;
}
