#![forbid(unsafe_code)]
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Result, Server};
use prep::{
    application::helper::get_configuration,
    domain::config::Settings,
    infra::{router, MySqlDatabase},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("----- Starting Server ------");
    let Settings {
        application_port,
        application_host,
        database_config,
    } = get_configuration();

    let address = format!("{}:{}", application_host, application_port);
    let listener = TcpListener::bind(address);
    let database = MySqlDatabase::new(&database_config).await;
    let router = router();

    Server::new(listener)
        .run(router.with(AddData::new(database)))
        .await
}
