use crate::{application::helper::get_configuration, domain::config::Settings};
use brize_auth::{
    auth::{auth_client::AuthClient, session_client::SessionClient},
    config::DatabaseConfig,
    mysql::MySqlGateway,
};

pub async fn auth() -> AuthClient<MySqlGateway> {
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

    AuthClient::new(&db_config).await
}

pub async fn session() -> SessionClient<MySqlGateway> {
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

    SessionClient::new(&db_config).await
}
