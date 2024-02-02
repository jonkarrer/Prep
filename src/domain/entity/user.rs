use chrono::{DateTime, Duration, Utc};
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

#[derive(FromRow, Deserialize, Serialize)]
pub struct PasswordResetToken {
    pub password_reset_token: String,
    pub password_reset_expiry: DateTime<Utc>,
}

impl PasswordResetToken {
    pub fn new() -> Self {
        let password_reset_token = uuid::Uuid::new_v4().to_string();
        let reset_duration = Duration::hours(1);
        let password_reset_expiry = Utc::now() + reset_duration;

        Self {
            password_reset_token,
            password_reset_expiry,
        }
    }

    pub fn match_token(&self, token: &str) -> bool {
        self.password_reset_token == token
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeleteAccountForm {
    pub csrf_token: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePasswordForm {
    pub csrf_token: String,
    pub new_password: String,
    pub current_password: String,
    pub current_email: String,
    pub reset_token: String,
}

#[derive(Deserialize)]
pub struct UpdateEmailForm {
    pub csrf_token: String,
    pub new_email: String,
}
