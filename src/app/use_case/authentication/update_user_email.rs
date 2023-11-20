use crate::{
    app::{clients::auth_client, interface::UserRepository},
    domain::entity::UpdateEmailForm,
};
use brize_auth::entity::Session;

pub async fn update_user_email<T: UserRepository>(
    session: &Session,
    repo: &T,
    form: &UpdateEmailForm,
) -> anyhow::Result<()> {
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
