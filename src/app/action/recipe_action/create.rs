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

#[cfg(test)]
mod tests {
    use crate::app::{
        clients::db_client,
        helper::{get_test_recipe_args, get_test_session},
    };

    use super::*;

    #[tokio::test]
    async fn test_case_create_recipe() {
        let repo = db_client().await;
        let recipe_args = get_test_recipe_args();
        let session = get_test_session().await.unwrap();

        let recipe = create_recipe(&repo, recipe_args, &session.user_id)
            .await
            .unwrap();

        assert_eq!(recipe.recipe_title, "Oatmeal")
    }
}
