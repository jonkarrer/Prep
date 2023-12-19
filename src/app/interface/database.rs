use crate::app::configs::DbConfig;
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct Database<T> {
    pub pool: T,
}

impl Database<MySqlPool> {
    pub async fn new(config: &DbConfig) -> Database<MySqlPool> {
        let addr = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user_name, config.password, config.host, config.port, config.db_name
        );
        let pool = MySqlPool::connect(addr.as_str())
            .await
            .expect("Failed connection with database");

        Database { pool }
    }
}
