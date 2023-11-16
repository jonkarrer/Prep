use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub struct UserId(pub String);

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub profile_pic_url: String,
    pub role: String,
}
