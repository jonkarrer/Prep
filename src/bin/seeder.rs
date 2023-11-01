use std::fs;

use prep::{configuration::get_configuration, domain::WebRecipe};
use sqlx::MySqlPool;

fn get_recipe_seed_data() -> Vec<WebRecipe> {
    let raw_data =
        fs::read_to_string("database/clean_recipes.json").expect("Could not find seed data file");

    let deserialized_data: Vec<WebRecipe> =
        serde_json::from_str(&raw_data).expect("Failed to deserialize");

    deserialized_data
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let configuration = get_configuration();
    // let db_configs = configuration.database;
    // let pool = MySqlPool::connect(db_configs.connection_string().as_str())
    //     .await
    //     .expect("Failed connection with database");

    dbg!(get_recipe_seed_data());

    // let recipe_id = uuid::Uuid::new_v4().to_string();
    // let user_id = uuid::Uuid::new_v4().to_string();

    // sqlx::query!(
    //     r#"
    //     INSERT INTO recipes (recipe_id, user_id, title, servings)
    //     VALUES (?,?,?,?)
    //     "#,
    //     recipe_id,
    //     user_id,
    //     "Oatmeal",
    //     2
    // )
    // .execute(&pool)
    // .await?;

    // sqlx::query!(
    //     r#"
    //     INSERT INTO ingredients (recipe_id, name, amount, unit)
    //     VALUES (?,?,?,?)
    //     "#,
    //     recipe_id,
    //     "oats",
    //     1.25,
    //     "cups"
    // )
    // .execute(&pool)
    // .await?;

    // sqlx::query!(
    //     r#"
    //     INSERT INTO directions (recipe_id, info, step_order)
    //     VALUES (?,?,?)
    //     "#,
    //     recipe_id,
    //     "mix oats with milk",
    //     1,
    // )
    // .execute(&pool)
    // .await?;

    // sqlx::query!(
    //     r#"
    //     INSERT INTO tags (recipe_id, name)
    //     VALUES (?,?)
    //     "#,
    //     recipe_id,
    //     "vegan",
    // )
    // .execute(&pool)
    // .await?;

    Ok(())
}
