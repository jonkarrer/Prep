use poem::{
    get, handler, listener::TcpListener, web::Json, web::Path, IntoResponse, Result, Route, Server,
};
use prep::application::{generate_recipe, RecipeRepository};
use prep::configuration::{get_configuration, DatabaseConfig};
use prep::domain::Recipe;
use prep::infra::MySqlGateway;

#[handler]
async fn make_recipe(Path(name): Path<String>) -> Result<String> {
    println!("request accepted");
    let recipe: Recipe = generate_recipe(name.as_str()).unwrap();

    let db_config = DatabaseConfig {
        host: "localhost".to_string(),
        password: "my-secret-pw".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        port: 3306,
    };

    let repo = MySqlGateway::new(&db_config).await;
    repo.insert(recipe, "jon@gmail").await?;

    Ok(String::from("Recipe inserted"))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // set configuration
    let configuration = get_configuration();
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address);

    println!("starting server");
    let app = Route::new().at("/make/recipe/:name", get(make_recipe));
    Server::new(listener).run(app).await
}
