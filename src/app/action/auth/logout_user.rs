use brize_auth::entity::Session;

use crate::app::clients::session_client;

pub async fn logout_user(session: &Session, csrf_token: &str) -> anyhow::Result<()> {
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
