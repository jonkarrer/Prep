#![forbid(unsafe_code)]
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Result, Server};
use prep::{
    application::helper::get_configuration,
    domain::config::Settings,
    infra::{db, router},
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
    let db = db().await;
    let router = router();

    Server::new(listener)
        .run(router.with(AddData::new(db)))
        .await
}
