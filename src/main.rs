use poem::{get, handler, listener::TcpListener, Result, Route, Server};
use prep::configuration::{get_configuration, Settings};

#[handler]
async fn health_check() -> Result<String> {
    Ok(String::from("All Good Here. Keep Going"))
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
