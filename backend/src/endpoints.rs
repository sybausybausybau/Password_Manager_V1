use axum::{Json, extract::{Path, State}, http::StatusCode};
use crate::{encryption::{decrypt_data, hash_password}, structs::{AppState, PasswordEntry, User}};
use uuid::Uuid;
use log::{debug, error, info};

pub async fn create_user(State(state): State<AppState>, Json(mut user) : Json<User>) -> Result<(StatusCode, String), (StatusCode, String)> { 
    user.id = Uuid::new_v4().to_string();
    
    user.hashed_master_password = hash_password(&user.hashed_master_password).map_err(|err| {  //* Hashed master password is not really hashed at this point, it's the master password sent by the client, but we will hash it before storing it in the database, so I keep the name of the field as is to avoid confusion
        error!("Failed to hash the master password : {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash the master password".to_string())
    })?;

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
        debug!("User with id {} doesn't exists", user_id);
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

pub async fn get_entries(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> Result<Json<Vec<PasswordEntry>>, (StatusCode, String)> {
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
        Some(mut user) => {
            let key = &user.hashed_master_password.as_bytes();

            for entry in &mut user.passwords {
                entry.password = decrypt_data(&entry.password, &entry.salt, &(key).to_vec())
                .map_err(|err| {
                    error!("Failed to decrypt password entry for user id {}, password entry : {:#?}, error {err}", user_id, entry);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get passwords".to_string())
                })?.into();
            }

            return Ok(Json(user.passwords))
        },
        None => return Err((StatusCode::NOT_FOUND, "Client doesn't exists.".to_string())),
    }
}

pub async fn delete_entry_of_user(State(state): State<AppState>, Path(user_id): Path<Uuid>, Json(password) : Json<PasswordEntry>) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = user_id.to_string();

    info!("User with id {} is deleting a passwords {:#?}", user_id, password);

    if !state.db.user_exists(&user_id).await.map_err(|err| { 
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })? {
        return Err((StatusCode::NOT_FOUND, "Client doesn't exists.".to_string()));
    }

    state.db.delete_entry(&user_id, password).await.map_err(|err| {
        error!("Database failure to delete password {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database Failure".to_string())

    })?;

    Ok(StatusCode::ACCEPTED)
}