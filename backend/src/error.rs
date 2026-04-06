use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Mongo Db error : {0}")]
    MongoDbError(#[from] mongodb::error::Error),
    #[error("libsodium error : {0}")]
    LibsodiumError(#[from] libsodium_rs::SodiumError),
    #[error("Unknown Error {0}")]
    UnknownError(String)
}