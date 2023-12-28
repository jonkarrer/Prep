mod verify_user_credentials;
pub use verify_user_credentials::*;

mod start_session_for_user;
pub use start_session_for_user::*;

mod login_user;
pub use login_user::*;

mod logout_user;
pub use logout_user::*;

mod register_new_user;
pub use register_new_user::*;

mod reset_user_password;
pub use reset_user_password::*;

mod update_user_email;
pub use update_user_email::*;

#[cfg(test)]
mod tests {
    use brize_auth::config::Expiry;

    use super::*;
    use crate::{
        app::{
            clients::{auth_client, db_client, session_client},
            helper::{TEST_USER_ID, TEST_USER_NAME, TEST_USER_PASSWORD},
            interface::UserRepository,
        },
        domain::entity::{PasswordResetToken, UpdateEmailForm, UpdatePasswordForm},
    };

    #[tokio::test]
    async fn test_action_register_new_user() {
        // Register user
        let random_str = &uuid::Uuid::new_v4().to_string();
        let user_name = &random_str[..10];
        let repo = db_client().await;
        let user_id = register_new_user(user_name, TEST_USER_PASSWORD, &repo)
            .await
            .unwrap();

        // Test register result
        let user = repo.get_user_by_id(&user_id.0).await.unwrap();
        assert_eq!(user.email, user_name);
    }

    #[tokio::test]
    async fn test_action_login_user() {
        // Login user
        let repo = db_client().await;
        let session = login_user(TEST_USER_NAME, TEST_USER_PASSWORD)
            .await
            .unwrap();

        // Test login result
        let user = repo.get_user_by_id(&session.user_id).await.unwrap();
        assert_eq!(user.email, TEST_USER_NAME);
    }

    #[tokio::test]
    async fn test_action_logout_user() {
        // Create a session
        let session_client = session_client().await;
        let session = session_client
            .start_session(TEST_USER_ID, Expiry::Month(1))
            .await
            .unwrap();

        // Run a logout
        logout_user(&session, &session.csrf_token).await.unwrap();

        // Test that session is no longer valid
        let check_session = session_client.validate_session(&session.session_id).await;
        assert!(check_session.is_err());
    }

    #[tokio::test]
    async fn test_action_reset_user_password() {
        // Register user
        let random_str = &uuid::Uuid::new_v4().to_string();
        let user_name = &random_str[..10];
        let repo = db_client().await;
        let user_id = register_new_user(user_name, TEST_USER_PASSWORD, &repo)
            .await
            .unwrap();

        // Create a session
        let session_client = session_client().await;
        let session = session_client
            .start_session(&user_id.0, Expiry::Month(1))
            .await
            .unwrap();

        // Create reset token
        let reset_token = PasswordResetToken::new();
        repo.insert_password_reset_token(&reset_token, &session.user_id)
            .await
            .unwrap();

        // Create rest password form
        let form = UpdatePasswordForm {
            csrf_token: session.csrf_token.to_string(),
            new_password: "test-new-password".to_string(),
            current_password: TEST_USER_PASSWORD.to_string(),
            current_email: user_name.to_string(),
            reset_token: reset_token.password_reset_token,
        };

        // Reset password
        reset_password(&session, &repo, &form).await.unwrap();

        // Test that session is no longer valid
        let auth = auth_client().await;
        let verify = auth
            .verify_credentials(user_name, "test-new-password")
            .await;

        assert!(verify.is_ok());
    }

    #[tokio::test]
    async fn test_action_update_user_email() {
        // Register user
        let random_str = &uuid::Uuid::new_v4().to_string();
        let user_name = &random_str[..10];
        let repo = db_client().await;
        let user_id = register_new_user(user_name, TEST_USER_PASSWORD, &repo)
            .await
            .unwrap();

        // Create a session
        let session_client = session_client().await;
        let session = session_client
            .start_session(&user_id.0, Expiry::Month(1))
            .await
            .unwrap();

        // Create update email form
        let random_str = &uuid::Uuid::new_v4().to_string();
        let user_email = &random_str[..10];
        let form = UpdateEmailForm {
            csrf_token: session.csrf_token.to_string(),
            new_email: user_email.to_string(),
        };

        // Reset password
        update_user_email(&session, &repo, &form).await.unwrap();

        // Test that session is no longer valid
        let auth = auth_client().await;
        let verify = auth
            .verify_credentials(user_email, TEST_USER_PASSWORD)
            .await;

        assert!(verify.is_ok());
    }
}
