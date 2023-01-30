mod handler;
mod db;
mod model;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;

use dotenv;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // let database = db::connector::init().await.unwrap();

    let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
    tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");

    let app = Router::new()
    .route("/user/signin", get(handler::user::signin))
    .route("/user/redirect", get(handler::user::kakao_redirect));
    // .with_state(database);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
