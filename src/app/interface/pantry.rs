use crate::domain::entity::PantryItem;
use anyhow::Result;

pub trait PantryRepository: Send + Sync {
    async fn select_all_pantry_items(&self, user_id: &str) -> Result<Vec<PantryItem>>;
    async fn create_pantry_item(&self, name: &str, user_id: &str) -> Result<PantryItem>;
}
