use domain::Recipe;
use anyhow::Result;

#[async_trait::async_trait]
pub trait RecipeRepository {
    async insert(recipe: &Recipe) -> Result<()>;
    async select(id: String) -> Result<Recipe>;
    async delete(id: String) -> Result
    async update(new_recipe: &Recipe) -> Result<()>;
}