use axum::{
    Router, routing::{get, post}
};

use backend::{endpoints::{add_entry_to_user, create_user}, error::ServerError, server_db::ServerDb, structs::AppState};
use log::{info};
use libsodium_rs::ensure_init;

#[tokio::main]
async fn main() -> Result<(), ServerError> {

    env_logger::init();
    ensure_init().expect("Failed to initialize libsodium");

    let state = AppState {
        db : ServerDb::new("mongodb://localhost:27017/").await?,
    };

    let app = Router::new()
        .route("/create_user", post(create_user))
        .route("/add_password/{id}", post(add_entry_to_user))
        .with_state(state);

    info!("Started server on port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}