use serde::{Deserialize, Serialize};
use libsodium_rs::{crypto_pwhash};
use crate::{encryption::{encrypt_data, hash_password}, error::ServerError, server_db::ServerDb};
use chrono::Local;

#[derive(Clone)]
pub struct AppState {
    pub db : ServerDb,
    pub jwt_secret : String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PasswordEntry {
    pub id: String,
    pub origin: String,
    pub username : String,
    pub password : Vec<u8>,
    pub salt : [u8; 16],
    pub added_time : i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id : String,
    pub username : String,
    pub hashed_master_password : String,
    pub passwords : Vec<PasswordEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordId {
    pub id : String
}

impl User {
    pub fn new(id : String, username: String, master_password: String) -> Result<Self, ServerError> {
        Ok(User {
            id, 
            username, 
            hashed_master_password : hash_password(master_password)?.into(),
            passwords: vec![],
            }
        )
    }

    pub fn add_password(&mut self, mut entry: PasswordEntry) -> Result<(), ServerError> {

        let key = self
            .hashed_master_password
            .as_bytes();

        let (password, salt)  = encrypt_data(&entry.password, key)?;

        entry.password = password;
        entry.salt = salt;

        self.passwords.push(entry); 
        Ok(())
    }

    pub fn delete_password(&mut self, id: &str) -> Result<(), ServerError> {
        if let Some(index) = self.passwords.iter().position(|x| x.id == id) {
            self.passwords.swap_remove(index);
            Ok(())
        } else {
            Err(ServerError::UnknownError(format!("Password for password id '{}' not found", id)))
        }
    }

    pub fn valid_master_password(&self, master_password: &str) -> Result<bool, ServerError> {
        Ok(crypto_pwhash::pwhash_str_verify(master_password, &self.hashed_master_password.as_bytes())?)
    }
}

impl PasswordEntry {
    pub fn new(id: String, origin: String, username: String, password : Vec<u8>) -> PasswordEntry {
        PasswordEntry {id, origin, username, password, salt : [0u8; 16], added_time: Local::now().timestamp()}
    }
} 