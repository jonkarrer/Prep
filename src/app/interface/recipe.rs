use crate::domain::entity::{Direction, Ingredient, Recipe, RecipeArgs, RecipeDetails, Tag};
use anyhow::Result;

pub trait RecipeRepository: Send + Sync {
    async fn create_recipe_from_args(
        &self,
        recipe_args: RecipeArgs,
        user_id: &str,
    ) -> Result<String>;
    async fn select_recipe_by_id(&self, recipe_id: &str) -> Result<Recipe>;
    async fn select_recipe_by_title(&self, recipe_title: &str, user_id: &str) -> Result<Recipe>;
    async fn select_all_recipes_details(&self, user_id: &str) -> Result<Vec<RecipeDetails>>;
    async fn select_ingredients_for_recipe(&self, recipe_id: &str) -> Result<Vec<Ingredient>>;
    async fn select_directions_for_recipe(&self, recipe_id: &str) -> Result<Vec<Direction>>;
    async fn select_tags_for_recipe(&self, recipe_id: &str) -> Result<Vec<Tag>>;
    async fn select_tags_for_user(&self, user_id: &str) -> Result<Vec<Tag>>;
    async fn select_recipe_details_by_id(&self, recipe_id: &str) -> Result<RecipeDetails>;
    async fn select_recipe_details_by_title(
        &self,
        recipe_title: &str,
        user_id: &str,
    ) -> Result<RecipeDetails>;
    async fn delete_recipe(&self, id: &str) -> Result<()>;
    async fn delete_ingredients_by_recipe_id(&self, recipe_id: &str) -> Result<()>;
    async fn delete_directions_by_recipe_id(&self, recipe_id: &str) -> Result<()>;
    async fn delete_tags_by_recipe_id(&self, recipe_id: &str) -> Result<()>;
    async fn update_recipe(
        &self,
        recipe_args: RecipeArgs,
        recipe_id: &str,
        user_id: &str,
    ) -> Result<()>;
}
