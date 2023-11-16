use brize_auth::entity::Session;
use poem::{
    web::{Data, Html},
    IntoResponse, Result,
};

#[poem::handler]
pub async fn handle_user_profile_details(
    Data(session): Data<&Session>,
) -> Result<impl IntoResponse> {
    Ok(Html(format!(
        r#"
        <div>{}</div>
    "#,
        session.user_id
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_user_profile() {
        let path = "/usr/profile/details";
        let app = Route::new().at(path, get(handle_user_profile_details));
        let test_client = TestClient::new(app);
        let resp = test_client.get(path).send().await;

        resp.assert_text("All Good Here. Keep Going").await;
    }
}
