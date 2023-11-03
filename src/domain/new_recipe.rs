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

pub fn get_test_recipe() -> NewRecipe {
    NewRecipe {
        title: "Oatmeal".to_string(),
        servings: 2.0,
        favorite: true,
        tags: vec!["vegan".to_string()],
        ingredients: vec![
            NewIngredient {
                name: "oats".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
            NewIngredient {
                name: "milk".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
        ],
        directions: vec![
            NewDirection {
                info: "boil and stir".to_string(),
                step_order: 1,
            },
            NewDirection {
                info: "enjoy and stir".to_string(),
                step_order: 1,
            },
        ],
    }
}
