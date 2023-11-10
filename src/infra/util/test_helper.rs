use crate::{
    domain::entity::{DirectionArgs, IngredientArgs, RecipeArgs},
    infra::authentication::auth,
};

pub async fn get_test_session_tokens() -> (String, String) {
    // get a session token
    let email = "seed_user@gmail.com";
    let password = "seeder_password";
    let mut auth = auth().await;
    auth.login(email, password).await.unwrap()
}

pub fn get_test_recipe_args() -> RecipeArgs {
    RecipeArgs {
        title: "Oatmeal".to_string(),
        servings: 2.0,
        favorite: true,
        tags: vec!["vegan".to_string()],
        ingredients: vec![
            IngredientArgs {
                name: "oats".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
            IngredientArgs {
                name: "milk".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
        ],
        directions: vec![
            DirectionArgs {
                details: "boil and stir".to_string(),
                step_order: 1,
            },
            DirectionArgs {
                details: "enjoy and stir".to_string(),
                step_order: 1,
            },
        ],
    }
}
