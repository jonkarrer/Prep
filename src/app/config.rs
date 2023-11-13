use crate::domain::config::{DatabaseConfig, Settings};
use std::env;

pub fn get_configuration() -> Settings {
    dotenvy::dotenv().unwrap_or_default();

    let database_config = DatabaseConfig {
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD not found"),
        user_name: env::var("DB_USER").expect("DB_USER not found"),
        host: env::var("DB_HOST").expect("DB_HOST not found"),
        port: env::var("DB_PORT").expect("DB_PORT not found"),
        db_name: env::var("DB_NAME").expect("DB_NAME not found"),
    };

    Settings {
        database_config,
        application_port: env::var("APP_PORT").expect("APP_PORT not found"),
        application_host: env::var("APP_HOST").expect("APP_HOST not found"),
    }
}
