use crate::app::clients::session_client;
use brize_auth::{config::Expiry, entity::Session};

pub async fn start_session_for_user(user_id: &str) -> anyhow::Result<Session> {
    session_client()
        .await
        .start_session(user_id, Expiry::Month(1))
        .await
}
