use crate::{application::repository::RecipeRepository, domain::Recipe};
use anyhow::Result;

pub async fn insert_new_recipe<R: RecipeRepository>(
    repo: &R,
    recipe: Recipe,
    user_id: &str,
) -> Result<String> {
    repo.insert(recipe, user_id).await
}
