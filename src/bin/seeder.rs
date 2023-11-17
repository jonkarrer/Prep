use prep::{
    app::clients::{auth_client, db_client},
    app::{
        configs::DbConfig,
        helper::{TEST_USER_NAME, TEST_USER_PASSWORD},
        interface::UserRepository,
    },
    domain::entity::RecipeArgs,
};
use sqlx::MySqlPool;
use std::fs;

fn get_recipe_seed_data() -> Vec<RecipeArgs> {
    let raw_data =
        fs::read_to_string("database/clean_recipes.json").expect("Could not find seed data file");

    let deserialized_data: Vec<RecipeArgs> =
        serde_json::from_str(&raw_data).expect("Failed to deserialize");

    deserialized_data
}

async fn seed_with_recipes() -> anyhow::Result<()> {
    let db_configs = DbConfig::default();
    let pool = MySqlPool::connect(db_configs.connection_string().as_str())
        .await
        .expect("Failed connection with database");

    // create one seed user
    let auth_client = auth_client().await;
    let creds_id = auth_client
        .register(TEST_USER_NAME, TEST_USER_PASSWORD)
        .await?;
    let user_id = db_client()
        .await
        .create_user(TEST_USER_NAME, &creds_id)
        .await?;

    // begin transaction
    let seed_data = get_recipe_seed_data();
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

async fn seed_with_users() -> anyhow::Result<()> {
    let users = vec![
        ("usr1@mail.com", "usr1password"),
        ("usr2@mail.com", "usr2password"),
        ("usr3@mail.com", "usr4password"),
        ("usr5@mail.com", "usr5password"),
        ("usr6@mail.com", "usr6password"),
    ];

    let auth_client = auth_client().await;
    for (email, pass) in users {
        let creds_id = auth_client.register(email, pass).await?;
        db_client().await.create_user(email, &creds_id).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    seed_with_recipes().await?;
    seed_with_users().await?;
    Ok(())
}
