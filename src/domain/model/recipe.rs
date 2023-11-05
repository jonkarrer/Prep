use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct RecipeModel {
    pub row_id: u64,
    pub recipe_id: String,
    pub user_id: String,
    pub recipe_title: String,
    pub created_at: String,
    pub updated_at: String,
    pub favorite: bool,
    pub servings: f32,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct IngredientModel {
    pub ingredient_id: u64,
    pub recipe_id: String,
    pub ingredient_name: String,
    pub amount: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct DirectionModel {
    pub direction_id: u64,
    pub recipe_id: String,
    pub direction_details: String,
    pub step_order: u16,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct TagsModel {
    pub tag_id: u64,
    pub recipe_id: String,
    pub tag_name: String,
}
