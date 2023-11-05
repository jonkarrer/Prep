use anyhow::Result;
use brize_auth::{Auth, AuthConfig, DatabaseConfig, Expiry, GatewayType, SessionType};
use serde::{Deserialize, Serialize};

use crate::application::{
    repository::{User, UserRepository},
    BasicAuth,
};

pub async fn register_new_user<U: UserRepository>(
    repo: &U,
    basic_auth: BasicAuth,
) -> Result<String> {
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

    let credentials_id: String = auth
        .register(&basic_auth.email, &basic_auth.password)
        .await?;

    let user_id = repo
        .create(&basic_auth.email, credentials_id.as_str())
        .await?;

    Ok(user_id)
}
