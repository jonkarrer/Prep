use poem::{get, handler, listener::TcpListener, Result, Route, Server};
use prep::application::RecipeRepository;
use prep::configuration::{get_configuration, Settings};
use prep::domain::Recipe;
use prep::infra::MySqlGateway;

#[handler]
async fn health_check() -> Result<String> {
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
        title: "Health Check".to_string(),
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

    let app = Route::new().at("/health_check", get(health_check));
    Server::new(listener).run(app).await
}
