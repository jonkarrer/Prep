use crate::{
    app::{
        clients::{auth_client, db_client, session_client},
        interface::UserRepository,
    },
    domain::entity::{DeleteAccountForm, UpdateEmailForm, UpdatePasswordForm, UserId},
};
use anyhow::Result;
use brize_auth::{config::Expiry, entity::Session};
use regex::Regex;

pub async fn login_user(user_name: &str, password: &str) -> Result<Session> {
    let user_id = verify_user_credentials(user_name, password).await?;
    start_session_for_user(&user_id.0).await
}

pub async fn logout_user(session: &Session, csrf_token: &str) -> Result<()> {
    match session.match_csrf_token(csrf_token) {
        true => {
            session_client()
                .await
                .destroy_session(&session.session_id)
                .await
        }
        false => Err(anyhow::anyhow!("Unauthorized")),
    }
}

pub async fn register_new_user<T: UserRepository>(
    user_name: &str,
    password: &str,
    repo: &T,
) -> Result<UserId> {
    let creds_id = auth_client().await.register(user_name, password).await?;
    let user_id = repo.create_user(user_name, creds_id.as_str()).await?;

    Ok(UserId(user_id))
}

pub async fn reset_password<T: UserRepository>(
    session: &Session,
    repo: &T,
    form: &UpdatePasswordForm,
) -> Result<()> {
    match session.match_csrf_token(&form.csrf_token) {
        true => {
            let auth = auth_client().await;

            // Validate current auth
            auth.verify_credentials(&form.current_email, &form.current_password)
                .await?;

            // Verify reset token is healthy and valid
            let reset_token = repo.get_password_reset_token(&session.user_id).await?;
            if reset_token.match_token(&form.reset_token) {
                // Update user password
                let user = repo.get_user_by_id(&session.user_id).await?;
                auth.update_password(&user.email, &form.new_password)
                    .await?;
            } else {
                return Err(anyhow::anyhow!("Unauthorized"));
            }

            Ok(())
        }
        false => Err(anyhow::anyhow!("Unauthorized")),
    }
}

pub async fn start_session_for_user(user_id: &str) -> Result<Session> {
    session_client()
        .await
        .start_session(user_id, Expiry::Month(1))
        .await
}

pub async fn update_user_email<T: UserRepository>(
    session: &Session,
    repo: &T,
    form: &UpdateEmailForm,
) -> Result<()> {
    if !validate_user_email(&form.new_email) {
        return Err(anyhow::anyhow!("Email is invalid"));
    }

    match session.match_csrf_token(&form.csrf_token) {
        true => {
            let auth = auth_client().await;
            let user = repo.get_user_by_id(&session.user_id).await?;
            let old_user_email = user.email;

            repo.update_email(&form.new_email, &session.user_id).await?;
            auth.update_user_name(&old_user_email, &form.new_email)
                .await?;

            Ok(())
        }
        false => Err(anyhow::anyhow!("Unauthorized")),
    }
}

pub async fn delete_account<T: UserRepository>(
    repo: &T,
    form: &DeleteAccountForm,
    session: &Session,
) -> Result<()> {
    match session.match_csrf_token(&form.csrf_token) {
        true => {
            let client = auth_client().await;

            // Validate current auth
            let user_id = verify_user_credentials(&form.email, &form.password).await?;

            // Delete from credentials table
            client.destroy_credentials(&form.email).await?;
            repo.delete_user(&user_id.0).await?;
            logout_user(session, &form.csrf_token).await?;

            Ok(())
        }
        false => Err(anyhow::anyhow!("Unauthorized")),
    }
}

pub fn validate_user_email(user_email: &str) -> bool {
    let email_re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
        .expect("Regex failed to init");

    email_re.is_match(user_email)
}

pub async fn verify_user_credentials(user_name: &str, password: &str) -> Result<UserId> {
    // verify credentials
    let client = auth_client().await;
    client.verify_credentials(user_name, password).await?;

    // get user id by user name
    let user = db_client().await.get_user_by_email(user_name).await?;

    Ok(UserId(user.user_id))
}

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
