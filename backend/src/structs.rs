use serde::{Deserialize, Serialize};
use libsodium_rs::crypto_pwhash;
use crate::{error::ServerError, server_db::ServerDb};
use chrono::Local;

#[derive(Clone)]
pub struct AppState {
    pub db : ServerDb
}

#[derive(Serialize, Deserialize)]
pub struct PasswordEntry {
    pub username : String,
    pub password : String,
    pub added_time : i64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id : String,
    pub username : String,
    pub hashed_master_password : String,
    pub passwords : Vec<PasswordEntry>,
}

pub fn hash_password(password: String) -> String {
    crypto_pwhash::pwhash_str(
        password.as_bytes(),
        crypto_pwhash::OPSLIMIT_INTERACTIVE,
        crypto_pwhash::MEMLIMIT_INTERACTIVE
    ).unwrap()
}

impl User {
    pub fn new(id: String, username: String, master_password: String) -> User {
        //* Storing the laster password hashed.
        let hash_str = hash_password(master_password);
        User {id: id, username: username, hashed_master_password: hash_str, passwords: vec![]}
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.push(PasswordEntry::new(username, password));
    }

    pub fn valid_master_password(&self, master_password: String) -> Result<bool, ServerError> {
        Ok(crypto_pwhash::pwhash_str_verify(&self.hashed_master_password, master_password.as_bytes())?)
    }
}

impl PasswordEntry {
    pub fn new(username: String, password : String) -> PasswordEntry {
        PasswordEntry {username: username, password: password, added_time: Local::now().timestamp()}
    }

    //pub fn hash_pwd(password, key) {}
}