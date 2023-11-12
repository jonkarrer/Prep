#![forbid(unsafe_code)]
use poem::{
    endpoint::StaticFilesEndpoint, http::StatusCode, listener::TcpListener, middleware::AddData,
    Endpoint, EndpointExt, IntoResponse, Request, Response, Result, Server,
};
use prep::{
    application::helper::get_configuration,
    domain::config::Settings,
    infra::{database::db, middleware::Log, router},
};

async fn catch_auth_error<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    println!("request: {}", req.uri().path());
    let res = next.call(req).await;

    match res {
        Ok(resp) => Ok(resp.into_response()),
        Err(err) => {
            if err.status() == StatusCode::UNAUTHORIZED {
                return Ok(Response::builder()
                    .header("Location", "/auth/login")
                    .status(StatusCode::SEE_OTHER)
                    .body("Unauthorized"));
            }
            Err(err)
        }
    }
}

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
        .with(AddData::new(db))
        .with(Log)
        .around(catch_auth_error);

    Server::new(listener).run(router).await
}
