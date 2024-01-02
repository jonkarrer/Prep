use crate::{app::interface::PantryRepository, domain::entity::PantryItem};

pub async fn create_pantry_item<T: PantryRepository>(
    repo: &T,
    item_name: &str,
    user_id: &str,
) -> anyhow::Result<PantryItem> {
    repo.create_pantry_item(item_name, user_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{clients::db_client, helper::get_test_user_id};

    #[tokio::test]
    async fn test_action_create_pantry_item() {
        let repo = db_client().await;
        let user_id = get_test_user_id().await;

        let pantry_item = create_pantry_item(&repo, "test_item", &user_id)
            .await
            .unwrap();

        assert_eq!(pantry_item.item_name, "test_item");
    }
}
