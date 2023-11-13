use crate::app::{configs::get_db_config, interface::Database};
use brize_auth::{config::DatabaseConfig, mysql::MySqlGateway, AuthClient, SessionClient};
use sqlx::MySqlPool;

fn client_config() -> DatabaseConfig {
    let d = get_db_config();
    DatabaseConfig {
        host: d.host,
        db_name: d.db_name,
        user_name: d.user_name,
        password: d.password,
        port: d.port,
        namespace: None,
    }
}

pub async fn session_client() -> SessionClient<MySqlGateway> {
    SessionClient::new(&client_config()).await
}

pub async fn auth_client() -> AuthClient<MySqlGateway> {
    AuthClient::new(&client_config()).await
}

pub async fn db_client() -> Database<MySqlPool> {
    Database::new(&get_db_config()).await
}
