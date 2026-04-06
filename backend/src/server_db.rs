use mongodb::Database;
use mongodb::{Client};

use crate::error::ServerError;
use crate::structs::{User};

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

    pub async fn add_user(&self, user : &User) -> Result<(), ServerError> {
        let collection = self.main_db.collection::<User>("users");
        collection.insert_one(user).await?;
        Ok(())
    }


}