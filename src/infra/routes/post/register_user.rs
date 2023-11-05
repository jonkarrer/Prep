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
    async fn test_route_register_user() {
        let app = Route::new().at("/register_user", post(register_user));
        let test_client = TestClient::new(app);

        let random_str = &uuid::Uuid::new_v4().to_string();
        let user_name = &random_str[..6];
        let email = &random_str[..10];

        let args = UserArgs {
            user_name: user_name.to_string(),
            password: "test_password".to_string(),
            email: email.to_string(),
        };
        let payload = serde_json::to_string(&args).unwrap();
        let mut resp = test_client
            .post("/register_user")
            .body(payload)
            .content_type("application/json")
            .send()
            .await;

        let id: String = resp.0.take_body().into_string().await.unwrap();
        dbg!(id);
        resp.assert_status_is_ok();
    }
}
