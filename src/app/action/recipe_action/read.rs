use crate::{
    app::interface::RecipeRepository,
    domain::entity::{Recipe, RecipeDetails},
};
use anyhow::Result;

pub async fn get_single_recipe<T: RecipeRepository>(repo: &T, recipe_id: &str) -> Result<Recipe> {
    repo.select_recipe_by_id(&recipe_id).await
}

pub async fn get_all_recipe_details_for_user<T: RecipeRepository>(
    repo: &T,
    user_id: &str,
) -> Result<Vec<RecipeDetails>> {
    repo.select_all_recipes_details(&user_id).await
}

#[cfg(test)]
mod tests {
    use crate::app::{
        clients::db_client,
        helper::{get_random_recipe_id, get_test_user_id},
    };

    use super::*;

    #[tokio::test]
    async fn test_action_get_single_recipe() {
        let recipe_id = get_random_recipe_id().await.unwrap();
        let repo = db_client().await;

        let recipe = get_single_recipe(&repo, &recipe_id).await.unwrap();

        assert!(recipe.recipe_title.len() != 0)
    }

    #[tokio::test]
    async fn test_action_get_all_recipe_details_for_user() {
        let user_id = get_test_user_id().await;
        let repo = db_client().await;

        let recipes = get_all_recipe_details_for_user(&repo, &user_id)
            .await
            .unwrap();

        assert!(recipes.len() != 0)
    }
}
