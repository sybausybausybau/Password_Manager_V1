use axum::{Json, extract::{Path, State}, http::StatusCode};
use crate::{encryption::{decrypt_data, hash_password}, structs::{AppState, PasswordEntry, User}};
use uuid::Uuid;
use log::{error, info};

pub async fn create_user(State(state): State<AppState>, Json(mut user) : Json<User>) -> Result<(StatusCode, String), (StatusCode, String)> { 
    user.id = Uuid::new_v4().to_string();
    
    let hash = hash_password(user.hashed_master_password.clone()).map_err(|err| {
        error!("Failed to hash the master password : {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash the master password".to_string())
    })?;

    user.hashed_master_password = hash;

    if let Err(err) = state.db.add_user(&user).await {
        error!( "Failed to store the user in the database, error : {}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to store the user in the database.".to_string()))
    }

    Ok((StatusCode::CREATED, user.id))
}

pub async fn add_entry_to_user(State(state): State<AppState>, Path(user_id): Path<Uuid>, Json(password) : Json<PasswordEntry>) -> Result<StatusCode, (StatusCode, String)> { 
    let user_id = user_id.to_string();

    info!("User with id {} is adding a passwords {:#?}", user_id, password);

    if !state.db.user_exists(&user_id).await.map_err(|err| { 
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })? {
        return Err((StatusCode::NOT_FOUND, "Client doesn't exists.".to_string()));
    }

    state.db.add_entry(&user_id, password).await.map_err(|err| {
        error!("Failed to add passwords to database : {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database Failure".to_string())
    })?;
    
    Ok(StatusCode::ACCEPTED)
}

pub async fn modify_entry_of_user(State(state): State<AppState>, Path(user_id): Path<Uuid>, Json(password) : Json<PasswordEntry>) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = user_id.to_string();

    info!("User with id {} is modifying a passwords {:#?}", user_id, password);

    if !state.db.user_exists(&user_id).await.map_err(|err| { 
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })? {
        return Err((StatusCode::NOT_FOUND, "Client doesn't exists.".to_string()));
    }

    state.db.modify_entry(&user_id, password).await.map_err(|err| {
        error!("Database failure to modify password {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database Failure".to_string())

    })?;

    Ok(StatusCode::ACCEPTED)

}

pub async fn get_password_entries(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> Result<Json<Vec<PasswordEntry>>, (StatusCode, String)> {
    let user_id = user_id.to_string();

    info!("User with id {}", user_id);

    if !state.db.user_exists(&user_id).await.map_err(|err| { 
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })? {
        return Err((StatusCode::NOT_FOUND, "Client doesn't exists.".to_string()));
    }

    match state.db.get_user(&user_id).await.map_err(|err| { 
        error!("Failed to get password entry : {err}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Database Failure".to_string()) 
    })? {
        Some(user) => {
            let key = &user.hashed_master_password
                .as_bytes()[..32]
                .try_into()
                .map_err(|e| {
                    error!("Invalid key for user {:#?}, {e}", user);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get passwords".to_string())
                })?;

            let decrypted_passwords = user.passwords.into_iter().map(|pwd_entry| {
                pwd_entry = decrypt_data(pwd_entry.password.as_bytes(), key)?;
                
                }).collect::<Vec<PasswordEntry>>();
            return Ok(Json(decrypted_passwords))
        },
        None => return Err((StatusCode::NOT_FOUND, "Client doesn't exists.".to_string())),
    }
}