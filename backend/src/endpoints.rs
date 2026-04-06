use axum::{Json, extract::State, http::StatusCode};
use crate::structs::{AppState, PasswordEntry, User};
use uuid::Uuid; 
use log::error;

pub async fn create_user(State(state): State<AppState>, Json(mut user) : Json<User>) -> Result<StatusCode, (StatusCode, String)> { 
    user.id = Uuid::new_v4().to_string();

    if let Err(err) = state.db.add_user(&user).await {
        error!( "Failed to store the user in the database, error : {}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to store the user in the database.".to_string()))
    }

    Ok(StatusCode::CREATED)
}

pub async fn add_passwords_to_user(State(state): State<AppState>, Json(mut passwords) : Json<Vec<PasswordEntry>>) -> Result<StatusCode, (StatusCode, String)> { 
    //Ok(StatusCode::CREATED)
    // TODO: Finish it
    todo!();

}
