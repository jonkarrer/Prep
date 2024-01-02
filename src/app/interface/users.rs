use crate::domain::entity::{PasswordResetToken, User};
use anyhow::Result;

pub trait UserRepository: Sync + Send {
    async fn create_user(&self, email: &str, credentials_id: &str) -> Result<String>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
    async fn get_user_by_id(&self, user_id: &str) -> Result<User>;
    async fn insert_password_reset_token(
        &self,
        token: &PasswordResetToken,
        user_id: &str,
    ) -> Result<()>;
    async fn update_email(&self, new_email: &str, user_id: &str) -> Result<()>;
    async fn get_password_reset_token(&self, user_id: &str) -> Result<PasswordResetToken>;
}
