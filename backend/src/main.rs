use axum::{
    Router, routing::{delete, get, post}
};

use backend::{
    endpoints::{
        add_entry_to_user, create_user, get_entries, modify_entry_of_user, delete_entry_of_user, login
    }, 
    error::ServerError, server_db::ServerDb, structs::AppState
};
use log::{error, info};
use libsodium_rs::ensure_init;

#[tokio::main]
async fn main() -> Result<(), ServerError> {

    unsafe { std::env::set_var("RUST_LOG", "trace") };

    if let Err(err) = dotenv::from_filename(r".env") {
        let msg = format!("Failed to retrieve .env file : {err}");
        error!("{msg}");

        return Err(ServerError::UnknownError(msg));
    }

    env_logger::init();
    ensure_init().expect("Failed to initialize libsodium");
    

    let state = AppState {
        db : ServerDb::new("mongodb://localhost:27017/").await?,
        jwt_secret : dotenv::var("JWT_SECRET").expect("Failed to load JWT Secret from .env file.")
    };

    let app = Router::new()
        .route("/create_user", post(create_user))
        .route("/add_entry/{token}", post(add_entry_to_user))
        .route("/get_entry_list/{token}", get(get_entries))
        .route("/modify_entry/{token}", post(modify_entry_of_user))
        .route("/delete_entry/{token}", delete(delete_entry_of_user))
        .route("/login", post(login))
        .with_state(state);

    info!("Started server on port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}