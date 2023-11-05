use crate::{
    application::repository::RecipeRepository,
    domain::{Recipe, RecipeArgs},
};
use anyhow::Result;

pub async fn store_user_recipe<R: RecipeRepository>(
    repo: &R,
    recipe: RecipeArgs,
    user_id: &str,
) -> Result<Recipe> {
    let recipe_id = repo.insert(recipe, user_id).await?;
    let recipe = repo.select_by_id(&recipe_id).await?;

    Ok(recipe)
}
