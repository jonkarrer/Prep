use crate::{
    application::interface::{Database, UserRepository},
    infra::{authentication::auth, service::BasicAuthParams},
};
use poem::{handler, http::StatusCode, web::Data, Error, Result};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_register_user(
    Data(repo): Data<&Database<MySqlPool>>,
    Data(basic_auth): Data<&BasicAuthParams>,
) -> Result<String> {
    let mut auth = auth().await;

    let credentials_id = auth
        .register(&basic_auth.email, &basic_auth.password)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::CONFLICT))?;

    let user_id = repo
        .create_user(&basic_auth.email, credentials_id.as_str())
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::BAD_GATEWAY))?;

    Ok(user_id)
}

#[cfg(test)]
mod tests {
    use crate::infra::{db, middleware::BasicAuth};

    use super::*;
    use base64::{engine::general_purpose, Engine};
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_register_user() {
        // build route
        let path = "/usr/register";
        let ep = Route::new()
            .at(path, post(handle_register_user))
            .with(BasicAuth)
            .with(AddData::new(db().await));
        let test_client = TestClient::new(ep);

        // create random user creds
        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password";

        // create bearer token
        let raw_token = format!("{}|{}", email, password);
        let encoded_token = general_purpose::STANDARD.encode(raw_token.as_bytes());
        let bearer_token = format!("Bearer {}", encoded_token);

        // run test
        let resp = test_client
            .post(path)
            .header("Authorization", bearer_token)
            .send()
            .await;

        // assert result
        resp.assert_status_is_ok();

        // TODO select by id in db to confirm registration
        // let id: String = resp.0.take_body().into_string().await.unwrap();
    }
}
