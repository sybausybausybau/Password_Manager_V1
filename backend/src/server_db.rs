use mongodb::Database;
use mongodb::{Client, bson::doc};
use crate::error::ServerError;
use crate::structs::{PasswordEntry, PasswordEntryClean, User};
//use log::info;

#[derive(Clone)]
pub struct ServerDb {
    main_db : Database
}

impl ServerDb {
    pub async fn new(url : &str) -> Result<Self, ServerError> {// mongodb://localhost:27017/
        let client = Client::with_uri_str(url).await?;
        let main_db = client.database("Password-Manager-V1");
        
        Ok(ServerDb { main_db })
    }

    pub async fn user_exists(&self, id : &str) -> Result<bool, ServerError> {
        let collection = self.main_db.collection::<User>("users");
        match collection.find_one(doc!{"id": id}).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn user_exists_by_username(&self, username : &str) -> Result<bool, ServerError> {
        let collection = self.main_db.collection::<User>("users");
        match collection.find_one(doc!{"username" : username}).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn get_user(&self, id : &str) -> Result<Option<User>, ServerError> {
        let collection = self.main_db.collection::<User>("users");
        match collection.find_one(doc!{"id": id}).await? {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    pub async fn add_user(&self, user : &User) -> Result<(), ServerError> {
        let collection = self.main_db.collection::<User>("users");
        collection.insert_one(user).await?;
        Ok(())
    }

    pub async fn add_entry(&self, id : &str, password : PasswordEntry) -> Result<(), ServerError> {
        let collection = self.main_db.collection::<User>("users");

        let mut user = collection
            .find_one(doc!{"id": id})
            .await?
            .ok_or(ServerError::UnknownError(format!("Cannot find user with id {id}")))?;

        user.add_password(password)?;

        collection.replace_one(doc! { "id" : id }, user).await?;
    
        Ok(())
    }

    pub async fn modify_entry(&self, user_id : &str, update: PasswordEntryClean) -> Result<(), ServerError>{
        let collection = self.main_db.collection::<User>("users");
        let mut user = collection
            .find_one(doc!{"id": user_id})
            .await?
            .ok_or(ServerError::UnknownError(format!("Cannot find user with id {user_id}")))?;
        

        let mut entry = user.find_password(&update.id)?;
        
        if let Some(username) = update.username {
            entry.username = username;
        }

        if let Some(new_password) = update.password {
            entry.password = new_password
        }

        user.delete_password(&update.id)?;

        user.add_password(entry)?;

        collection.replace_one(doc!{"id": user_id}, user).await?;

        Ok(())
    }

    pub async fn delete_entry(&self, id : &str, password_id: &str) -> Result<(), ServerError>{
        let collection = self.main_db.collection::<User>("users");
        let mut user = collection
            .find_one(doc!{"id": id})
            .await?
            .ok_or(ServerError::UnknownError(format!("Cannot find user with id {id}")))?;
        
        user.delete_password(password_id)?;

        collection.replace_one(doc!{"id": id}, user).await?;

        Ok(())
    }

    pub async fn get_user_from_username(&self, username: &str) -> Result<Option<User>, ServerError>{
        let collection = self.main_db.collection::<User>("users");
        let user = collection
            .find_one(doc!{"username": &username})
            .await?;
        Ok(user)
    }
}