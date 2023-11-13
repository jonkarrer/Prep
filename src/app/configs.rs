use std::env;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub app_port: String,
    pub app_host: String,
}

impl Settings {
    pub fn default() -> Self {
        dotenvy::dotenv().unwrap_or_default();

        Settings {
            app_port: env::var("APP_PORT").expect("APP_PORT not found"),
            app_host: env::var("APP_HOST").expect("APP_HOST not found"),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DbConfig {
    pub user_name: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db_name: String,
}

impl DbConfig {
    pub fn default() -> DbConfig {
        dotenvy::dotenv().unwrap_or_default();

        DbConfig {
            password: env::var("DB_PASSWORD").expect("DB_PASSWORD not found"),
            user_name: env::var("DB_USER").expect("DB_USER not found"),
            host: env::var("DB_HOST").expect("DB_HOST not found"),
            port: env::var("DB_PORT").expect("DB_PORT not found"),
            db_name: env::var("DB_NAME").expect("DB_NAME not found"),
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user_name, self.password, self.host, self.port, self.db_name
        )
    }
}
