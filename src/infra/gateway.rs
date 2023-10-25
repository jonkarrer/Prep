use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use application::interface::RecipeRepository;
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

pub struct RecipeRecord {
    id: Option<Thing>
    recipe: Recipe
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

impl RecipeRepository for SurrealGateway {
    async fn insert(&self, recipe: &Recipe, user_id: &str) -> Result<()> {
        let record = RecipeRecord {
            id: None,
            recipe
        };

        self.database
            .create::<Option<RecipeRecord>>(("recipes", user_id))
            .content(&record)
            .await
            .context("Failed to insert recipe")?;

        Ok(())
    }
}