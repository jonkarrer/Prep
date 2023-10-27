use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Recipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
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
