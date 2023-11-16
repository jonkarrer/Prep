use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct UserModel {
    pub row_id: u32,
    pub user_id: String,
    pub email: String,
    pub credential_id: String,
    pub password_reset_token: Option<String>,
    pub password_reset_expiry: Option<String>,
    pub last_login: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub profile_pic_url: Option<String>,
    pub role: String,
}
