use anyhow::Context;

use crate::{
    app::interface::RecipeRepository,
    domain::entity::{Recipe, RecipeArgs},
};

pub async fn create_recipe<T: RecipeRepository>(
    repo: &T,
    recipe_args: RecipeArgs,
    user_id: &str,
) -> anyhow::Result<Recipe> {
    let recipe_id = repo.create_recipe_from_args(recipe_args, user_id).await?;
    repo.select_recipe_by_id(&recipe_id).await
}
