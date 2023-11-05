use prep::{configuration::get_configuration, domain::RecipeArgs};
use sqlx::MySqlPool;
use std::fs;

fn get_recipe_seed_data() -> Vec<RecipeArgs> {
    let raw_data =
        fs::read_to_string("database/clean_recipes.json").expect("Could not find seed data file");

    let deserialized_data: Vec<RecipeArgs> =
        serde_json::from_str(&raw_data).expect("Failed to deserialize");

    deserialized_data
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration();
    let db_configs = configuration.database;
    let pool = MySqlPool::connect(db_configs.connection_string().as_str())
        .await
        .expect("Failed connection with database");

    let seed_data = get_recipe_seed_data();
    let user_id = uuid::Uuid::new_v4().to_string();

    // begin transaction
    let mut transaction = pool.begin().await.expect("Transaction failed to start");
    for recipe in seed_data {
        let recipe_id = uuid::Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO recipes (recipe_id, user_id, recipe_title, servings)
            VALUES (?,?,?,?)
            "#,
            recipe_id,
            user_id,
            recipe.title,
            recipe.servings
        )
        .execute(&mut *transaction)
        .await?;

        for ingredient in recipe.ingredients {
            sqlx::query!(
                r#"
                INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit)
                VALUES (?,?,?,?)
                "#,
                recipe_id,
                ingredient.name,
                ingredient.amount,
                ingredient.unit
            )
            .execute(&mut *transaction)
            .await?;
        }

        for direction in recipe.directions {
            sqlx::query!(
                r#"
                INSERT INTO directions (recipe_id, direction_details, step_order)
                VALUES (?,?,?)
                "#,
                recipe_id,
                direction.details,
                direction.step_order,
            )
            .execute(&mut *transaction)
            .await?;
        }

        for tag in recipe.tags {
            sqlx::query!(
                r#"
                INSERT INTO tags (recipe_id, tag_name)
                VALUES (?,?)
                "#,
                recipe_id,
                tag,
            )
            .execute(&mut *transaction)
            .await?;
        }
    }

    transaction
        .commit()
        .await
        .expect("Failed to commit transaction");

    Ok(())
}
