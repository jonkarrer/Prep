use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AiGeneratedRecipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub directions: Vec<String>,
    pub servings: f32,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
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

pub fn convert_to_recipe(content: &str) -> AiGeneratedRecipe {
    serde_json::from_str(&content).expect("Failed to parse dat")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Direction {
    pub step_order: u16,
    pub info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub amount: f32,
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub title: String,
    pub servings: f32,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub directions: Vec<Direction>,
    pub ingredients: Vec<Ingredient>,
}

pub fn get_test_recipe() -> Recipe {
    Recipe {
        title: "Oatmeal".to_string(),
        servings: 2.0,
        favorite: true,
        tags: vec!["vegan".to_string()],
        ingredients: vec![
            Ingredient {
                name: "oats".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
            Ingredient {
                name: "milk".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
        ],
        directions: vec![
            Direction {
                info: "boil and stir".to_string(),
                step_order: 1,
            },
            Direction {
                info: "enjoy and stir".to_string(),
                step_order: 1,
            },
        ],
    }
}
