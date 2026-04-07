use axum::{
    Router, routing::{post}
};
use backend::{endpoints::{add_entry_to_user, create_user, get_entries, modify_entry_of_user, delete_entry_of_user}, error::ServerError, server_db::ServerDb, structs::AppState};
use log::{info};
use libsodium_rs::ensure_init;

#[tokio::main]
async fn main() -> Result<(), ServerError> {

    env_logger::init();
    ensure_init().expect("Failed to initialize libsodium");
    unsafe { std::env::set_var("RUST_LOG", "debug") };
    let state = AppState {
        db : ServerDb::new("mongodb://localhost:27017/").await?,
    };

    let app = Router::new()
        .route("/create_user", post(create_user))
        .route("/add_entry/{id}", post(add_entry_to_user))
        .route("/get_entry_list/{id}", post(get_entries))
        .route("/modify_entry/{id}", post(modify_entry_of_user))
        .route("/delete_user/{id}", post(delete_entry_of_user))
        .with_state(state);

    info!("Started server on port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}