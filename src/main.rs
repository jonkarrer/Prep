mod application;
mod domain;

use domain::Recipe;
use application::generate_recipe;
use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server, web::Json};

#[handler]
fn make_recipe(Path(name): Path<String>) -> Json<Recipe> {
    println!("request accepted");
    Json(generate_recipe(name.as_str()).unwrap())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("starting server");
    let app = Route::new().at("/make/recipe/:name", get(make_recipe));
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
}
