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
    async fn insert(&self, recipe: Recipe, user_id: &str) -> Result<()> {
        let record = RecipeRecord {
            id: None,
            user_id: user_id.to_string(),
            recipe,
        };

        self.db
            .create::<Vec<RecipeRecord>>("recipes")
            .content(&record)
            .await
            .context("Failed to insert recipe")?;

        Ok(())
    }

    async fn select(&self, id: &str) -> Result<Recipe> {
        let query_for_record: Option<RecipeRecord> = self.db.select(("recipes", id)).await?;

        match query_for_record {
            Some(record) => Ok(record.recipe),
            None => Err(anyhow::anyhow!("Failed to select recipe by id")),
        }
    }

    async fn update(&self, new_recipe: Recipe, id: &str) -> Result<()> {
        self.db
            .update::<Option<RecipeRecord>>(("recipes", id))
            .content(new_recipe)
            .await
            .context("Failed to update recipe")?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        self.db
            .delete::<Option<RecipeRecord>>(("recipes", id))
            .await
            .context("Failed to delete recipe")?;

        Ok(())
    }
}
