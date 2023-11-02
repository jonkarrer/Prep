use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Recipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub servings: f32,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct WebDirection {
    pub step_order: u16,
    pub info: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct WebIngredient {
    pub name: String,
    pub amount: f32,
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct WebRecipe {
    pub title: String,
    pub servings: f32,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub directions: Vec<WebDirection>,
    pub ingredients: Vec<WebIngredient>,
}

pub struct RecipeModel {
    pub row_id: u64,
    pub recipe_id: String,
    pub user_id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub favorite: bool,
}

pub struct IngredientModel {
    pub ingredient_id: u64,
    pub recipe_id: String,
    pub name: String,
    pub amount: f64,
    pub unit: String,
}

pub struct DirectionModel {
    pub direction_id: u64,
    pub recipe_id: String,
    pub info: String,
    pub step_order: u16,
}

pub struct TagsModel {
    pub tag_id: u64,
    pub recipe_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecipeRecord {
    pub id: String,
    pub user_id: String,
    pub recipe: serde_json::Value,
}

pub fn convert_to_recipe(content: &str) -> Recipe {
    serde_json::from_str(&content).expect("Failed to parse dat")
}
