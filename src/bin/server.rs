use axum::routing::{get, post};
use axum::{Router, Server};
use puzzle15_backend::handler::record::{create_record, list_record};
use puzzle15_backend::handler::user::{login, register};
use puzzle15_backend::ping;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/hello", get(ping))
        .nest(
            "/user",
            Router::new()
                .route("/register", post(register))
                .route("/login", post(login)),
        )
        .nest(
            "/record",
            Router::new()
                .route("/create", post(create_record))
                .route("/list", post(list_record)),
        );

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
