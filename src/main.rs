#![forbid(unsafe_code)]
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Result, Server};
use prep::{
    application::helper::get_configuration,
    domain::config::Settings,
    infra::{
        database::db,
        middleware::{ErrorCatcher, Log},
        router,
    },
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
    let router = router().with(AddData::new(db)).with(ErrorCatcher).with(Log);

    Server::new(listener).run(router).await
}
