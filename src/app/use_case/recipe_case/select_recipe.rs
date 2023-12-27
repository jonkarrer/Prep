use crate::{app::interface::RecipeRepository, domain::entity::Recipe};

pub async fn select_recipe<T: RecipeRepository>(
    repo: &T,
    recipe_id: &str,
) -> anyhow::Result<Recipe> {
    repo.select_recipe_by_id(&recipe_id).await
}
