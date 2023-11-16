use poem::Result;

#[poem::handler]
pub async fn handle_health_check() -> Result<String> {
    Ok(String::from("All Good Here. Keep Going"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_health_check() {
        let app = Route::new().at("/health_check", get(handle_health_check));
        let test_client = TestClient::new(app);
        let resp = test_client.get("/health_check").send().await;

        resp.assert_text("All Good Here. Keep Going").await;
    }
}
