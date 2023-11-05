use anyhow::Result;
use brize_auth::{Auth, AuthConfig, DatabaseConfig, Expiry, GatewayType, SessionType};
use serde::{Deserialize, Serialize};

use crate::application::repository::{User, UserRepository};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserArgs {
    pub user_name: String,
    pub password: String,
    pub email: String,
}

pub async fn register_new_user<U: UserRepository>(repo: &U, user_args: UserArgs) -> Result<String> {
    let db_config = DatabaseConfig {
        host: "localhost:3306".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        password: "my-secret-pw".to_string(),
        namespace: None,
    };

    let config = AuthConfig::new()
        .set_credentials_gateway(GatewayType::MySql(db_config))
        .set_session_type(SessionType::Session(Expiry::Month(1)));

    let mut auth = Auth::new(config).await.unwrap();

    let credentials_id: String = auth.register(&user_args.email, &user_args.password).await?;

    let user_id = repo
        .create(
            &user_args.user_name,
            &user_args.email,
            credentials_id.as_str(),
        )
        .await?;

    Ok(user_id)
}
