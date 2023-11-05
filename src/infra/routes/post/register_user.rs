use poem::{handler, web::Json, Result};

use crate::{
    application::{register_new_user, UserArgs},
    configuration::{get_configuration, Settings},
    infra::MySqlGateway,
};

#[handler]
pub async fn register_user(Json(user_args): Json<UserArgs>) -> Result<String> {
    let Settings { database, .. } = get_configuration();
    let repo = MySqlGateway::new(&database).await;

    let user_id = register_new_user(&repo, user_args).await?;

    Ok(user_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{post, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_create_recipe() {
        let app = Route::new().at("/register_user", post(register_user));
        let test_client = TestClient::new(app);

        let args = UserArgs {
            user_name: "test_register".to_string(),
            password: "test_password".to_string(),
            email: "test_regi@email.com".to_string(),
        };
        let payload = serde_json::to_string(&args).unwrap();
        let resp = test_client
            .post("/register_user")
            .body(payload)
            .content_type("application/json")
            .send()
            .await;

        resp.assert_status_is_ok();
        let id: String = resp.0.take_body().into_string().await.unwrap();
        dbg!(id);
        assert!(false);
    }
}
