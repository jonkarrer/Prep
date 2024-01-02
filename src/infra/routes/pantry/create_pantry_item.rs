use crate::{
    app::{action::create_pantry_item, interface::Database},
    domain::entity::PantryItem,
};
use brize_auth::entity::Session;
use poem::{
    handler,
    web::{Data, Json},
    Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_create_pantry_item(
    item_name: String,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<Json<PantryItem>> {
    let pantry_item = create_pantry_item(repo, &item_name, &session.user_id).await?;

    Ok(Json(pantry_item))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::clients::db_client;
    use crate::app::helper::get_test_session;
    use crate::domain::constants::SESSION_COOKIE_KEY;
    use crate::infra::middleware::AuthGuard;
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_create_pantry_item() {
        // build route
        let path = "/pantry/create";
        let ep = Route::new()
            .at(path, post(handle_create_pantry_item))
            .with(AddData::new(db_client().await))
            .with(AuthGuard);

        let test_client = TestClient::new(ep);

        // get a session token
        let session = get_test_session().await;

        // run test
        let resp = test_client
            .post(path)
            .body("test_pantry_creation_route")
            .header(
                "Cookie",
                format!("{}={}", SESSION_COOKIE_KEY, session.session_id),
            )
            .header("X-CSRF-Token", session.csrf_token)
            .content_type("application/json")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: PantryItem = resp.json().await.value().deserialize();
        assert_eq!(json.item_name, "test_pantry_creation_route");
    }
}
