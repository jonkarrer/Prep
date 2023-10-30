use poem::{
    get, handler, listener::TcpListener, web::Json, web::Path, IntoResponse, Result, Route, Server,
};
use prep::application::{generate_recipe, RecipeRepository};
use prep::configuration::{get_configuration, DatabaseConfig, Settings};
use prep::domain::Recipe;
use prep::infra::MySqlGateway;

#[handler]
async fn make_recipe(Path(name): Path<String>) -> Result<String> {
    println!("request accepted");
    // let recipe: Recipe = generate_recipe(name.as_str()).unwrap();

    let recipe = Recipe {
        ingredients: vec![
            "1 1/2 pounds ground beef".to_string(),
            "1/2 cup breadcrumbs".to_string(),
        ],
        instructions: vec![
            "Preheat the oven to 350°F (175°C).".to_string(),
            "In a large bowl, combine all the ingredients.".to_string(),
        ],
        title: name,
        servings: 1.0,
    };

    let Settings { database, .. } = get_configuration();

    dbg!(&database);
    let repo = MySqlGateway::new(&database).await;
    dbg!("repo connected");
    repo.insert(recipe, "jon@gmail").await?;

    Ok(String::from("Recipe inserted"))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("----- Starting Server -----");
    let Settings {
        application_port,
        application_host,
        ..
    } = get_configuration();

    let address = format!("{}:{}", application_host, application_port);
    let listener = TcpListener::bind(address);

    let app = Route::new().at("/make/recipe/:name", get(make_recipe));
    Server::new(listener).run(app).await
}
