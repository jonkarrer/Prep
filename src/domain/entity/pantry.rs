use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct PantryItem {
    pub user_id: String,
    pub name: String,
    pub in_stock: bool,
}
