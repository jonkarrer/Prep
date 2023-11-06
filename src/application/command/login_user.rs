use crate::application::{helper::init_auth_client, BasicAuth};
use anyhow::Result;

pub async fn login_user(basic_auth: BasicAuth) -> Result<String> {
    let mut auth = init_auth_client().await?;
    let session_token: String = auth.login(&basic_auth.email, &basic_auth.password).await?;
    Ok(session_token)
}
