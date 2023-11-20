use crate::{
    app::{clients::auth_client, interface::UserRepository},
    domain::entity::UserId,
};

pub async fn register_new_user<T: UserRepository>(
    user_name: &str,
    password: &str,
    repo: &T,
) -> anyhow::Result<UserId> {
    let creds_id = auth_client().await.register(user_name, password).await?;
    let user_id = repo.create_user(user_name, creds_id.as_str()).await?;

    Ok(UserId(user_id))
}
