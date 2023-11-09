#![forbid(unsafe_code)]
use poem::{
    endpoint::StaticFilesEndpoint, listener::TcpListener, middleware::AddData, EndpointExt, Result,
    Server,
};
use prep::{
    application::helper::get_configuration,
    domain::config::Settings,
    infra::{db, middleware::Log, router},
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
    let router = router()
        .nest(
            "/",
            StaticFilesEndpoint::new("./")
                .show_files_listing()
                .index_file("index.html"),
        )
        .at(
            "/dashboard",
            StaticFilesEndpoint::new("./")
                .show_files_listing()
                .index_file("./templates/index.html"),
        )
        .with(AddData::new(db))
        .with(Log);

    Server::new(listener).run(router).await
}
