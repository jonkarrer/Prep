use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Tag {
    pub tag_id: u32,
    pub tag_name: String,
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Direction {
    pub direction_id: u32,
    pub step_order: u16,
    pub direction_details: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Ingredient {
    pub ingredient_id: u32,
    pub ingredient_name: String,
    pub amount: f32,
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub recipe_id: String,
    pub recipe_title: String,
    pub servings: f32,
    pub favorite: bool,
    pub tags: Vec<Tag>,
    pub directions: Vec<Direction>,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeMetadata {
    pub recipe_id: String,
    pub recipe_title: String,
    pub servings: f32,
    pub favorite: bool,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct RecipeDetails {
    pub recipe_id: String,
    pub recipe_title: String,
    pub servings: f32,
    pub favorite: bool,
}
