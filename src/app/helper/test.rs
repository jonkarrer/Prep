use crate::{
    app::clients::{db_client, session_client},
    app::interface::UserRepository,
    domain::entity::{DirectionArgs, IngredientArgs, RecipeArgs},
};
use anyhow::Result;
use brize_auth::{config::Expiry, entity::Session};

pub const TEST_USER_NAME: &str = "seed_user@gmail.com";
pub const TEST_USER_PASSWORD: &str = "seeder_password";

pub async fn get_test_session() -> Result<Session> {
    let user = db_client().await.get_user_by_email(TEST_USER_NAME).await?;

    session_client()
        .await
        .start_session(&user.user_id, Expiry::Month(1))
        .await
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
