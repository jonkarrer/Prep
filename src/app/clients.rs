use crate::app::{configs::DbConfig, interface::Database};
use brize_auth::{mysql::MySqlGateway, AuthClient, SessionClient};
use sqlx::MySqlPool;

pub async fn session_client() -> SessionClient<MySqlGateway> {
    SessionClient::new_mysql_client(&DbConfig::from_url()).await
}

pub async fn auth_client() -> AuthClient<MySqlGateway> {
    AuthClient::new_mysql_client(&DbConfig::from_url()).await
}

pub async fn db_client() -> Database<MySqlPool> {
    Database::new(&DbConfig::from_url()).await
}
