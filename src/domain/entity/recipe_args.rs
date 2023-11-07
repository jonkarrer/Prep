use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DirectionArgs {
    pub step_order: u16,
    pub details: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IngredientArgs {
    pub name: String,
    pub amount: f32,
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeArgs {
    pub title: String,
    pub servings: f32,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub directions: Vec<DirectionArgs>,
    pub ingredients: Vec<IngredientArgs>,
}
