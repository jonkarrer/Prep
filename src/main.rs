#![forbid(unsafe_code)]
use poem::{listener::TcpListener, Result, Server};
use prep::{
    configuration::{get_configuration, Settings},
    infra::router,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("----- Starting Server ------");
    let Settings {
        application_port,
        application_host,
        ..
    } = get_configuration();

    let address = format!("{}:{}", application_host, application_port);
    let listener = TcpListener::bind(address);
    let router = router();

    Server::new(listener).run(router).await
}
