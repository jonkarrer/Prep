use anyhow::Result;

#[async_trait::async_trait]
pub trait UserRepository: Sync + Send {
    async fn create(&self, email: &str, credentials_id: &str) -> Result<String>;
    // async fn select_by_id(&self, user_id: &str) -> Result<User>;
}
