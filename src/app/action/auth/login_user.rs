use super::{start_session_for_user, verify_user_credentials};
use brize_auth::entity::Session;

pub async fn login_user(user_name: &str, password: &str) -> anyhow::Result<Session> {
    let user_id = verify_user_credentials(user_name, password).await?;
    start_session_for_user(&user_id.0).await
}
