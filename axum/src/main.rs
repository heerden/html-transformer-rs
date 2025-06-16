use crate::api::routes;
use axum::serve;
use tokio::net::TcpListener;

mod api;

#[tokio::main]
async fn main() {
    let app = routes();
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Could not find the main TCP listener");
    serve(listener, app)
        .await
        .expect("Failed to start the server");
}
