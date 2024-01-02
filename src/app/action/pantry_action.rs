use crate::{app::interface::PantryRepository, domain::entity::PantryItem};

pub async fn create_pantry_item<T: PantryRepository>(
    repo: &T,
    item_name: &str,
    user_id: &str,
) -> anyhow::Result<PantryItem> {
    repo.create_pantry_item(item_name, user_id).await
}
