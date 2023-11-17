use crate::{
    app::{clients::auth_client, interface::UserRepository},
    domain::entity::UpdatePasswordForm,
};
use brize_auth::entity::Session;

pub async fn reset_password<T: UserRepository>(
    session: &Session,
    repo: &T,
    form: &UpdatePasswordForm,
) -> anyhow::Result<()> {
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
