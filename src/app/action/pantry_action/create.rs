use crate::{app::interface::PantryRepository, domain::entity::PantryItem};

pub async fn create_pantry_item<T: PantryRepository>(
    repo: &T,
    pantry_item_name: &str,
    user_id: &str,
) -> anyhow::Result<PantryItem> {
    let item_name = repo.create_pantry_item(pantry_item_name, user_id).await?;

    Ok(PantryItem {
        user_id: user_id.to_string(),
        name: item_name,
        in_stock: false,
    })
}
