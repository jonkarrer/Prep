use crate::{domain::Recipe, infra::RecipeRecord};
use anyhow::Result;

#[async_trait::async_trait]
pub trait RecipeRepository {
    type RecipeId;
    async fn insert(&self, recipe: Recipe, user_id: &str) -> Result<Self::RecipeId>;
    async fn select_by_id(&self, id: &str) -> Result<Recipe>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn update(&self, new_recipe: Recipe, id: &str) -> Result<()>;
}
