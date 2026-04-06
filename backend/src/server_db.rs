use mongodb::Database;
use mongodb::{Client, bson::doc};
use crate::error::ServerError;
use crate::structs::{PasswordEntry, User};
use log::info;

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

    pub async fn user_already_exists(&self, id : &str) -> Result<bool, ServerError> {
        let collection = self.main_db.collection::<User>("users");
        match collection.find_one(doc!{"id": id}).await? {
            Some(_) => Ok(true),
            None => Ok(false),
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
    // TODO : A finir
    pub async fn modify_entry(&self, id : &str, password: PasswordEntry) -> Result<(), ServerError>{
        let collection = self.main_db.collection::<User>("users");
        let mut user = collection
            .find_one(doc!{"id": id})
            .await?
            .ok_or(ServerError::UnknownError(format!("Cannot find user with id {id}")))?;
        
        user.delete_password(&password.id)?;

        user.add_password(password)?;

        collection.replace_one(doc!{"id": id}, user).await?;

        Ok(())
    }
}