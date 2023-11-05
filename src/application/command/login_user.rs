use crate::application::{auth, BasicAuth};
use anyhow::Result;

pub async fn login_user(basic_auth: BasicAuth) -> Result<String> {
    let mut auth = auth().await?;
    let session_token: String = auth.login(&basic_auth.email, &basic_auth.password).await?;
    Ok(session_token)
}
