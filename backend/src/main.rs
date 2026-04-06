use axum::{
    routing::get,
    Router,
};

use backend::{endpoints::create_user, error::ServerError, server_db::ServerDb, structs::AppState};
use log::{info};

#[tokio::main]
async fn main() -> Result<(), ServerError> {

    env_logger::init();

    let state = AppState {
        db : ServerDb::new("mongodb://localhost:27017/").await?,
    };

    let app = Router::new()
        .route("/create_user", get(create_user))
        .with_state(state);

    info!("Started server on port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}