use super::MySqlGateway;
use crate::configuration::{get_configuration, Settings};

pub async fn database() -> MySqlGateway {
    let Settings { database, .. } = get_configuration();
    MySqlGateway::new(&database).await
}
