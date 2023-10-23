use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    println!("request accepted");
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("starting server");
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
}
