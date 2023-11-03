use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDirection {
    pub step_order: u16,
    pub info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewIngredient {
    pub name: String,
    pub amount: f32,
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRecipe {
    pub title: String,
    pub servings: f32,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub directions: Vec<NewDirection>,
    pub ingredients: Vec<NewIngredient>,
}
