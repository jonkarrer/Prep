use crate::domain::config::DatabaseConfig;

use super::{RecipeRepository, UserRepository};

#[derive(Clone)]
pub struct Database<T> {
    pub pool: T,
}

#[async_trait::async_trait]
pub trait DatabaseConn<T>: UserRepository + RecipeRepository {
    async fn new(config: &DatabaseConfig) -> Database<T>;
}
