use crate::{app::interface::RecipeRepository, domain::entity::Recipe};

pub async fn get_single_recipe<T: RecipeRepository>(
    repo: &T,
    recipe_id: &str,
) -> anyhow::Result<Recipe> {
    repo.select_recipe_by_id(&recipe_id).await
}

pub async fn get_all_recipes_for_user<T: RecipeRepository>(
    repo: &T,
    recipe_id: &str,
) -> anyhow::Result<Recipe> {
    repo.select_recipe_by_id(&recipe_id).await
}

#[cfg(test)]
mod tests {
    use crate::app::{clients::db_client, helper::get_random_recipe_id};

    use super::*;

    #[tokio::test]
    async fn test_case_select_recipe() {
        let recipe_id = get_random_recipe_id().await.unwrap();
        let repo = db_client().await;

        let recipe = get_single_recipe(&repo, &recipe_id).await.unwrap();

        assert!(recipe.recipe_title.len() != 0)
    }
}
