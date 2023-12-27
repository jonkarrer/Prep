use crate::{
    app::interface::RecipeRepository,
    domain::entity::{RecipeArgs, RecipeDetails},
};

pub async fn create_recipe<T: RecipeRepository>(
    repo: &T,
    recipe_args: RecipeArgs,
    user_id: &str,
) -> anyhow::Result<RecipeDetails> {
    let recipe_id = repo.create_recipe_from_args(recipe_args, user_id).await?;
    repo.select_recipe_details_by_id(&recipe_id).await
}
