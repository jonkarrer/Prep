use poem::{get, handler, listener::TcpListener, test::TestClient, Result, Route, Server};
use prep::{
    configuration::{get_configuration, Settings},
    infra::app_router,
};

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
    Server::new(listener).run(app_router()).await
}
