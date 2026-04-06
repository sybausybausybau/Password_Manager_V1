use serde::{Deserialize, Serialize};
use libsodium_rs::crypto_pwhash;
use crate::{encryption::{encrypt_data, hash_password}, error::ServerError, server_db::ServerDb};
use chrono::Local;

#[derive(Clone)]
pub struct AppState {
    pub db : ServerDb
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PasswordEntry {
    pub id: String,
    pub origin: String,
    pub username : String,
    pub password : String,
    pub added_time : i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id : String,
    pub username : String,
    pub hashed_master_password : String,
    passwords : Vec<PasswordEntry>,
}

impl User {
    pub fn new(id : String, username: String, master_password: String) -> Result<Self, ServerError> {
        Ok(User {
            id, 
            username, 
            hashed_master_password : hash_password(master_password)?,
            passwords: vec![],
            }
        )
    }

    pub fn add_password(&mut self, mut entry: PasswordEntry) -> Result<(), ServerError> {
        let key: &[u8; 32] = self
            .hashed_master_password
            .as_bytes()[0..32]
            .try_into()
            .expect("slice with incorrect length");

        entry.password = String::from_utf8_lossy(&encrypt_data(entry.password.as_bytes(), key)?).to_string();
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

    pub fn valid_master_password(&self, master_password: String) -> Result<bool, ServerError> {
        Ok(crypto_pwhash::pwhash_str_verify(&self.hashed_master_password, master_password.as_bytes())?)
    }
}

impl PasswordEntry {
    pub fn new(id: String, origin: String, username: String, password : String) -> PasswordEntry {
        PasswordEntry {id, origin, username, password, added_time: Local::now().timestamp()}
    }
} 