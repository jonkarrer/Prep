use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct Database<T> {
    pub pool: T,
}

impl Database<MySqlPool> {
    pub async fn new(db_url: &str) -> Database<MySqlPool> {
        let pool = MySqlPool::connect(db_url)
            .await
            .expect("Failed connection with database");

        Database { pool }
    }
}
