use crate::{
    app::interface::{Database, UserRepository},
    domain::entity::{PasswordResetToken, User},
};
use anyhow::{Context, Result};
use sqlx::mysql::MySqlPool;

#[async_trait::async_trait]
impl UserRepository for Database<MySqlPool> {
    async fn create_user(&self, email: &str, credentials_id: &str) -> Result<String> {
        let user_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO users (user_id, email, credential_id)
            VALUES (?,?,?)
            "#,
        )
        .bind(&user_id)
        .bind(email)
        .bind(credentials_id)
        .execute(&self.pool)
        .await
        .context("Failed to create user in database")?;

        Ok(user_id)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        sqlx::query_as(
            r#"
            SELECT user_id, email, profile_pic_url, role
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find user_id by email in database")
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<User> {
        sqlx::query_as(
            r#"
            SELECT user_id, email, profile_pic_url, role
            FROM users
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find user_id by email in database")
    }

    async fn insert_password_reset_token(
        &self,
        token: &PasswordResetToken,
        user_id: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users 
            SET password_reset_token = ?, password_reset_expiry = ?
            WHERE user_id = ?
            "#,
        )
        .bind(token.password_reset_token.as_str())
        .bind(token.password_reset_expiry)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .context("Failed to update reset password details")?;

        Ok(())
    }

    async fn get_password_reset_token(&self, user_id: &str) -> Result<PasswordResetToken> {
        sqlx::query_as(
            r#"
            SELECT password_reset_token, password_reset_expiry
            FROM users
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to select recipe details by id")
    }

    async fn update_email(&self, new_email: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET email = ?
            WHERE user_id = ?
            "#,
        )
        .bind(new_email)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
