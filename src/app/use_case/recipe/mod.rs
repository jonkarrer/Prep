use rand::Rng;

use crate::app::{clients::db_client, helper::TEST_USER_ID, interface::RecipeRepository};

// TODO create a meals generator function
pub async fn get_random_recipes() {
    let db = db_client().await;
    let recipes = db
        .select_all_recipe_details_for_user(TEST_USER_ID)
        .await
        .unwrap();

    // TODO handle dupes
    // TODO desserts won't count as meals
    let mut randoms = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let r = rng.gen_range(0..recipes.len());
        randoms.push(&recipes[r]);
    }

    dbg!(randoms);
}

pub async fn get_all_unique_ingredients() {
    let db = db_client().await;
    let _recipes = db
        .select_all_recipe_details_for_user(TEST_USER_ID)
        .await
        .unwrap();
}
