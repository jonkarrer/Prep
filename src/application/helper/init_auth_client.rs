use anyhow::{Context, Result};
use brize_auth::{
    auth::{Auth, AuthBuilder},
    config::{DatabaseConfig, Expiry, SessionType},
    mysql::MySqlGateway,
};

pub async fn init_auth_client() -> Result<Auth<MySqlGateway, MySqlGateway>> {
    let db_config = DatabaseConfig {
        host: "localhost".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        password: "my-secret-pw".to_string(),
        port: "3306".to_string(),
        namespace: None,
    };

    let auth = AuthBuilder::new()
        .set_credentials_db_config(&db_config)
        .set_sessions_db_config(&db_config)
        .set_session_type(SessionType::Session(Expiry::Month(1)))
        .build()
        .await
        .context("Failed to build auth")?;

    Ok(auth)
}
