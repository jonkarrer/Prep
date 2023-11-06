use brize_auth::{
    auth::{Auth, AuthBuilder},
    config::{DatabaseConfig, Expiry, SessionType},
    mysql::MySqlGateway,
};

use crate::{application::helper::get_configuration, domain::config::Settings};

pub async fn auth() -> Auth<MySqlGateway, MySqlGateway> {
    let Settings {
        database_config, ..
    } = get_configuration();

    let db_config = DatabaseConfig {
        host: database_config.host,
        db_name: database_config.db_name,
        user_name: database_config.user_name,
        password: database_config.password,
        port: database_config.port,
        namespace: None,
    };

    AuthBuilder::new()
        .set_credentials_db_config(&db_config)
        .set_sessions_db_config(&db_config)
        .set_session_type(SessionType::Session(Expiry::Month(1)))
        .build()
        .await
        .expect("Failed to build auth")
}
