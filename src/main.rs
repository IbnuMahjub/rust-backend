mod db;
mod handlers;
mod models;
mod routes;

use axum::Router;
use dotenvy::dotenv;
use routes::api::api_routes;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().merge(api_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server jalan di http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
