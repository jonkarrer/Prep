use anyhow::{Context, Result};
use brize_auth::{Auth, AuthConfig, DatabaseConfig, Expiry, GatewayType, SessionType};

pub async fn auth() -> Result<Auth> {
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

    let auth = Auth::new(config).await.context("Failed to init auth")?;

    Ok(auth)
}
