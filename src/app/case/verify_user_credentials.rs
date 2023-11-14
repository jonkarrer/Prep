use crate::{
    app::{
        clients::{auth_client, db_client},
        interface::UserRepository,
    },
    domain::entity::UserId,
};
use anyhow::Result;

pub async fn verify_user_credentials(user_name: &str, password: &str) -> Result<UserId> {
    // verify credentials
    let client = auth_client().await;
    client.verify_credentials(user_name, password).await?;

    // get user id by user name
    let user = db_client().await.get_user_by_email(user_name).await?;

    Ok(UserId(user.user_id))
}
