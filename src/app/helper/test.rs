use crate::{
    app::clients::{db_client, session_client},
    app::interface::{RecipeRepository, UserRepository},
    domain::entity::{DirectionArgs, IngredientArgs, RecipeArgs},
};
use anyhow::Result;
use brize_auth::{config::Expiry, entity::Session};

pub const TEST_USER_NAME: &str = "seed_user@gmail.com";
pub const TEST_USER_PASSWORD: &str = "seeder_password";
pub const TEST_USER_ID: &str = "8e73026e-b4c1-425b-b8f8-6d006406f31d";

pub async fn get_test_session() -> Result<Session> {
    let user = db_client().await.get_user_by_email(TEST_USER_NAME).await?;

    session_client()
        .await
        .start_session(&user.user_id, Expiry::Month(1))
        .await
}

pub async fn get_test_user_id() -> Result<String> {
    let session = get_test_session().await?;
    Ok(session.user_id)
}

pub async fn get_random_recipe_id() -> Result<String> {
    let db = db_client().await;
    let user = db.get_user_by_email(TEST_USER_NAME).await?;

    let all_recipes = db.select_all_recipes_details(&user.user_id).await?;

    Ok(all_recipes[0].recipe_id.to_string())
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
