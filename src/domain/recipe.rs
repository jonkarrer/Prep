use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

pub fn convert_to_recipe(content: &str) -> Recipe {
    serde_json::from_str(&content).expect("Failed to parse dat")
}
