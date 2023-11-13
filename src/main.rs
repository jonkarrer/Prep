#![forbid(unsafe_code)]
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Result, Server};
use prep::{
    app::configs::{get_settings, Settings},
    infra::{
        clients::db_client,
        middleware::{ErrorCatcher, Log},
        routes::router,
    },
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("----- Starting Server ------");
    let Settings {
        application_port,
        application_host,
    } = get_settings();

    let address = format!("{}:{}", application_host, application_port);
    let listener = TcpListener::bind(address);
    let db = db_client().await;
    let router = router().with(AddData::new(db)).with(ErrorCatcher).with(Log);

    Server::new(listener).run(router).await
}
