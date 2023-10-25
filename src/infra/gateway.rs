use crate::{application::RecipeRepository, domain::Recipe};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RecipeRecord {
    id: Option<Thing>,
    user_id: String,
    recipe: Recipe,
}

pub struct SurrealGateway {
    pub db: Surreal<Client>,
}

impl SurrealGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let db = Surreal::new::<Ws>(config.host.as_str())
            .await
            .expect("Failed connection with SurrealDB");

        db.signin(Root {
            username: config.user_name.as_str(),
            password: config.password.as_str(),
        })
        .await
        .expect("Failed to sign into SurrealDB");

        let namespace = match &config.namespace {
            Some(namespace) => namespace.as_str(),
            None => "",
        };

        db.use_ns(namespace)
            .use_db(config.db_name.as_str())
            .await
            .expect("Failed connection with SurrealDB");

        Self { db }
    }
}

#[async_trait::async_trait]
impl RecipeRepository for SurrealGateway {
    type RecipeId = String;
    async fn insert(&self, recipe: Recipe, user_id: &str) -> Result<Self::RecipeId> {
        let record = RecipeRecord {
            id: None,
            user_id: user_id.to_string(),
            recipe,
        };

        let recipe_record: Vec<RecipeRecord> = self
            .db
            .create("recipes")
            .content(&record)
            .await
            .context("Failed to insert recipe")?;

        match recipe_record.get(0) {
            Some(record) => match &record.id {
                Some(id) => Ok(id.id.to_raw()),
                None => Err(anyhow::anyhow!("Recipe missing an id")),
            },
            None => Err(anyhow::anyhow!("Recipe not found")),
        }
    }

    async fn select_by_id(&self, recipe_id: &str) -> Result<Recipe> {
        let query_for_record: Option<RecipeRecord> = self.db.select(("recipes", recipe_id)).await?;

        match query_for_record {
            Some(record) => Ok(record.recipe),
            None => Err(anyhow::anyhow!("Failed to select recipe by recipe_id")),
        }
    }

    async fn update(&self, new_recipe: Recipe, recipe_id: &str) -> Result<()> {
        self.db
            .update::<Option<RecipeRecord>>(("recipes", recipe_id))
            .content(new_recipe)
            .await
            .context("Failed to update recipe")?;

        Ok(())
    }

    async fn delete(&self, recipe_id: &str) -> Result<()> {
        self.db
            .delete::<Option<RecipeRecord>>(("recipes", recipe_id))
            .await
            .context("Failed to delete recipe")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_surreal_gateway() {
        let recipe = Recipe {
            ingredients: vec![
                "1 1/2 pounds ground beef".to_string(),
                "1/2 cup breadcrumbs".to_string(),
            ],
            instructions: vec![
                "Preheat the oven to 350°F (175°C).".to_string(),
                "In a large bowl, combine all the ingredients.".to_string(),
            ],
            title: "Classic Meatloaf".to_string(),
        };

        let db_config = DatabaseConfig {
            db_name: "test".to_string(),
            host: "127.0.0.1:3000".to_string(),
            user_name: "root".to_string(),
            password: "surreal_ps".to_string(),
            namespace: Some("test".to_string()),
        };

        let repo = SurrealGateway::new(&db_config).await;

        // Test insert
        let id = repo.insert(recipe, "jon@gmail").await.unwrap();

        // Test select by recipe id
        let recipe = repo.select_by_id(id.as_str()).await.unwrap();
        assert_eq!(recipe.title, "Classic Meatloaf");
    }
}
