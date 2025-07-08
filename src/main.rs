use axum::{response::Html, routing::get, Router};
mod system;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/status", get(system::get_status));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}