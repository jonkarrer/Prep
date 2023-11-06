use crate::{
    application::repository::{RecipeRepository, UserRepository},
    domain::config::DatabaseConfig,
};

#[async_trait::async_trait]
pub trait DatabaseGateway: RecipeRepository + UserRepository {
    async fn new(config: &DatabaseConfig) -> Self;
}
