#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub user_name: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db_name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user_name, self.password, self.host, self.port, self.db_name
        )
    }
}
