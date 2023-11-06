use anyhow::Result;

pub enum UserRoles {
    User,
    Admin,
    Editor,
}
#[allow(dead_code)]
pub struct User {
    row_id: u32,
    user_id: String,
    email: String,
    credential_id: String,
    password_reset_token: Option<String>,
    password_reset_expiry: Option<String>,
    last_login: Option<String>,
    created_at: String,
    updated_at: String,
    profile_pic_url: Option<String>,
    role: UserRoles,
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(&self, email: &str, credentials_id: &str) -> Result<String>;
    // async fn select_by_id(&self, user_id: &str) -> Result<User>;
}
