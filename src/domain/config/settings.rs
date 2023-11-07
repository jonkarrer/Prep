use super::database::DatabaseConfig;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database_config: DatabaseConfig,
    pub application_port: String,
    pub application_host: String,
}
