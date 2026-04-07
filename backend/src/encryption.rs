use libsodium_rs::{crypto_aead::xchacha20poly1305::{Key, Nonce, encrypt, decrypt, NPUBBYTES},
                   crypto_pwhash::{argon2i::{MEMLIMIT_INTERACTIVE, OPSLIMIT_INTERACTIVE, pwhash_str}}
};

use crate::error::ServerError;

pub fn hash_password(password: String) -> Result<String, ServerError> {
    Ok(pwhash_str(
        password.as_bytes(),
        OPSLIMIT_INTERACTIVE,
        MEMLIMIT_INTERACTIVE
    )?.as_str().to_string())
}

pub fn encrypt_data(data : &[u8], key : &[u8; 32]) -> Result<Vec<u8>, ServerError> {
    let nonce = Nonce::generate();
    let mut encrypted_data = encrypt(
        data,
        None,
        &nonce,
        &Key::from_bytes(key)?,
    )?;

    encrypted_data.extend_from_slice(nonce.as_bytes());

    Ok(encrypted_data)
}

pub fn decrypt_data(crypted_data : &[u8], key : &[u8; 32]) -> Result<String, ServerError> {
    if crypted_data.len() <= 24 {
        return Err(ServerError::UnknownError("Invalid Nonce".to_owned()))
    }
    let nonce: [u8; 24] = crypted_data[crypted_data.len() - NPUBBYTES..].try_into().expect("Invalid Nonce");
    
    let decrypted_data = decrypt(
        &crypted_data[..crypted_data.len() - NPUBBYTES],
        None,
        &Nonce::from_bytes(nonce),
        &Key::from_bytes(key)?
    )?;
    
    Ok(String::from_utf8_lossy(&decrypted_data).to_string())
}

