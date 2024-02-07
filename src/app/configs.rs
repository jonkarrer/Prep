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
    pub port: Option<String>,
    pub db_name: String,
}

impl DbConfig {
    pub fn default() -> DbConfig {
        dotenvy::dotenv().unwrap_or_default();

        DbConfig {
            password: env::var("DB_PASSWORD").expect("DB_PASSWORD not found"),
            user_name: env::var("DB_USER").expect("DB_USER not found"),
            host: env::var("DB_HOST").expect("DB_HOST not found"),
            port: env::var("DB_PORT").ok(),
            db_name: env::var("DB_NAME").expect("DB_NAME not found"),
        }
    }

    pub fn connection_string() -> String {
        dotenvy::dotenv().unwrap_or_default();

        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not found");
        let user_name = env::var("DB_USER").expect("DB_USER not found");
        let host = env::var("DB_HOST").expect("DB_HOST not found");
        let port = env::var("DB_PORT").ok();
        let db_name = env::var("DB_NAME").expect("DB_NAME not found");

        match port {
            Some(port) => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    user_name, password, host, port, db_name
                )
            }
            None => {
                format!("mysql://{}:{}@{}/{}", user_name, password, host, db_name)
            }
        }
    }

    pub fn from_url() -> String {
        dotenvy::dotenv().unwrap_or_default();
        env::var("DATABASE_URL").expect("DATABASE_URL not found")
    }
}

pub struct StaticPath(pub String);

impl StaticPath {
    pub fn from(path: &str) -> Self {
        dotenvy::dotenv().unwrap_or_default();

        let prefix = env::var("STATIC_FILE_PREFIX").expect("STATIC_FILE_PREFIX not found");

        Self(format!("{}{}", prefix, path))
    }

    pub fn root() -> Self {
        dotenvy::dotenv().unwrap_or_default();

        let prefix = env::var("STATIC_FILE_PREFIX").expect("STATIC_FILE_PREFIX not found");
        Self(prefix.to_string())
    }
}
