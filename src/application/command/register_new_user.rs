use crate::application::{helper::init_auth_client, repository::UserRepository, BasicAuth};
use anyhow::Result;

pub async fn register_new_user<U: UserRepository>(
    repo: &U,
    basic_auth: BasicAuth,
) -> Result<String> {
    let mut auth = init_auth_client().await?;

    let credentials_id: String = auth
        .register(&basic_auth.email, &basic_auth.password)
        .await?;

    let user_id = repo
        .create(&basic_auth.email, credentials_id.as_str())
        .await?;

    Ok(user_id)
}
