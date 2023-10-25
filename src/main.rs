mod application;
mod domain;
mod infra;

use domain::Recipe;
use application::generate_recipe;
use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server, web::Json};

#[handler]
fn make_recipe(Path(name): Path<String>) -> Result<String> {
    println!("request accepted");
    let recipe: Recipe = generate_recipe(name.as_str()).unwrap();

    let db_config = DatabaseConfig {
        db_name: "test".to_string(),
        host: "127.0.0.1:3000".to_string(),
        user_name: "root".to_string(),
        password: "surreal_ps".to_string(),
        namespace: Some("test".to_string()),
    };

    let repo = SurrealGateway::new(&db_config).await;
    repo.insert(&recipe, "jon@gmail.com").await?;

    Ok(String::from("Recipe inserted"))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("starting server");
    let app = Route::new().at("/make/recipe/:name", get(make_recipe));
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await

      Start database
}
