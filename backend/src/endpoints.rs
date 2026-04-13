use axum::{Json, extract::State, http::{HeaderMap, HeaderValue, StatusCode}};
use crate::{auth::{create_cookie, create_token, get_token}, encryption::{decrypt_data, hash_password}, structs::{AppState, Credential, PasswordEntry, PasswordId, User, PasswordEntryClean}};
use uuid::Uuid;
use log::{debug, error, info};

fn internal_error<E: std::fmt::Display>(err: E, msg: &str) -> (StatusCode, String) {
    error!("{}: {}", msg, err); // Log l'erreur pour le serveur
    (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()) // Message pour le client
}

pub async fn create_user(State(state): State<AppState>, Json(mut user) : Json<User>) -> Result<(StatusCode, HeaderMap), (StatusCode, String)> { 
    user.id = Uuid::new_v4().to_string();
    
    let user_exists = state.db
        .user_exists_by_username(&user.username)
        .await
        .map_err(|err| internal_error(err, "Failed to check if user exists"))?;

    if user_exists {
        debug!("{} already exists", user.username);
        return Err((StatusCode::NOT_ACCEPTABLE, format!("User with username {} already exists", user.username)));
    }

    user.hashed_master_password = hash_password(&user.hashed_master_password).map_err(|err| {  //* Hashed master password is not really hashed at this point, it's the master password sent by the client, but we will hash it before storing it in the database, so I keep the name of the field as is to avoid confusion
        error!("Failed to hash the master password : {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash the master password".to_string())
    })?;

    if let Err(err) = state.db.add_user(&user).await {
        error!( "Failed to store the user in the database, error : {}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to store the user in the database.".to_string()))
    }

    let token = create_token(user.id, &state.jwt_secret, 30 * 60)
        .await
        .map_err(|err| internal_error(err, "Failed to create token for user"))?;

    info!("token for {} : {}", &user.username, &token);

    let cookie = create_cookie(token)
        .await
        .map_err(|err| internal_error(err, "Failed to create a cookie for user"))?
        .to_string();

    let cookie = HeaderValue::from_str(&cookie)
        .map_err(|err| {
            error!("Failed to create cookie : {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create a token.".to_string())
        })?;

    let mut headers = HeaderMap::new();
        
    headers.append("Set-Cookie", cookie);

    Ok((StatusCode::CREATED, headers))
}

pub async fn add_entry_to_user(State(state): State<AppState>, headers : HeaderMap, Json(mut password) : Json<PasswordEntry>) -> Result<StatusCode, (StatusCode, String)> { 
    let token = get_token(headers, &state.jwt_secret)
        .await
        .map_err(|err| internal_error(err, "Failed to decode token"))?;
    
    let user_id = token.claims.sub.to_string();

    info!("User with id {} is adding a passwords {:#?}", user_id, password);

    if !state.db.user_exists(&user_id).await.map_err(|err| { 
        debug!("User with id {} doesn't exists", user_id);
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })? {
        return Err((StatusCode::NOT_FOUND, "Client doesn't exists.".to_string()));
    }

    password.id = uuid::Uuid::new_v4().to_string();

    state.db.add_entry(&user_id, password).await.map_err(|err| {
        error!("Failed to add passwords to database : {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database Failure".to_string())
    })?;
    
    Ok(StatusCode::ACCEPTED)
}

pub async fn modify_entry_of_user(State(state): State<AppState>, headers: HeaderMap, Json(password) : Json<PasswordEntryClean>) -> Result<StatusCode, (StatusCode, String)> {
    let token = get_token(headers, &state.jwt_secret)
        .await
        .map_err(|err| internal_error(err, "Failed to decode token"))?;
    
    let user_id = token.claims.sub.to_string();

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

pub async fn get_entries(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<Vec<PasswordEntry>>, (StatusCode, String)> {
    let token = get_token(headers, &state.jwt_secret)
        .await
        .map_err(|err| internal_error(err, "Failed to decode token"))?;
    
    let user_id = token.claims.sub.to_string();

    info!("User with id {} is getting his password entries", user_id);

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
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Client doesn't exists.".to_string())),
    }
}

pub async fn delete_entry_of_user(State(state): State<AppState>, headers: HeaderMap, Json(password_id) : Json<PasswordId>) -> Result<StatusCode, (StatusCode, String)> {
    let token = get_token(headers, &state.jwt_secret)
        .await
        .map_err(|err| internal_error(err, "Failed to decode token"))?;
    
    let user_id = token.claims.sub.to_string();
    let password_id = password_id.id;

    info!("User with id {} is deleting a password with id {:?}", user_id, password_id);

    if !state.db.user_exists(&user_id).await.map_err(|err| { 
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })? {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Client doesn't exists.".to_string()));
    }

    state.db.delete_entry(&user_id, &password_id).await.map_err(|err| {
        error!("Database failure to delete password {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database Failure".to_string())
    })?;

    Ok(StatusCode::ACCEPTED)
}

pub async fn login(State(state): State<AppState>, Json(cred): Json<Credential>) -> Result<HeaderMap, (StatusCode, String)> {
    let user = state.db
        .get_user_from_username(&cred.username)
        .await
        .map_err(|err| internal_error(err, "Failed to get user with unique username"))?;

    let user = match user {
        Some(user) => Ok(user),
        None => Err((StatusCode::NOT_FOUND, "Could not find user with username".to_string())),
    }?;

    let valid = user
        .valid_master_password(&cred.master_password)
        .map_err(|err| internal_error(err, "Failed to verify master password"))?;

    if !valid {
        return Err((StatusCode::FORBIDDEN, "Invalid credentials".to_string()))
    }

    let token = create_token(user.id, &state.jwt_secret, cred.exp)
        .await
        .map_err(|err| internal_error(err, "Failed to create cookies"))?;

    info!("token for {} : {}", &cred.username, &token);

    let cookie = create_cookie(token)
        .await
        .map_err(|err| internal_error(err, "Failed to create cookies"))?
        .to_string();
    
    let cookie = HeaderValue::from_str(&cookie)
        .map_err(|err| {
            error!("Failed to create cookie : {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create a token.".to_string())
        })?;

    let mut header = HeaderMap::new();
    
    header.append("Set-Cookie", cookie);
    
    Ok(header)
}