#![forbid(unsafe_code)]

use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Result, Server};
use prep::{
    app::{clients::db_client, configs::Settings},
    infra::{
        middleware::{ErrorCatcher, Log},
        routes::router,
    },
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("----- Starting Server ------");
    let settings = Settings::default();
    let address = format!("{}:{}", settings.app_host, settings.app_port);

    let listener = TcpListener::bind(address);
    let db = db_client().await;
    let router = router().with(AddData::new(db)).with(ErrorCatcher).with(Log);

    Server::new(listener).run(router).await
}
