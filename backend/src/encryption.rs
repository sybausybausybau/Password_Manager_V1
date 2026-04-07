use libsodium_rs::{crypto_aead::xchacha20poly1305::{Key, NPUBBYTES, Nonce, decrypt, encrypt},
                   crypto_pwhash::{self, argon2i::{MEMLIMIT_INTERACTIVE, OPSLIMIT_INTERACTIVE, SALTBYTES, pwhash_str}}, random::{fill_bytes}
};

use crate::error::ServerError;

pub fn hash_password<T: AsRef<[u8]>>(password : T) -> Result<String, ServerError> {
    Ok(pwhash_str(
        password.as_ref(),
        OPSLIMIT_INTERACTIVE,
        MEMLIMIT_INTERACTIVE
    )?)
}

pub fn encrypt_data<T: AsRef<[u8]>>(data : T, key : &[u8]) -> Result<(Vec<u8>, [u8; 16]), ServerError> {  
    let mut salt = [0u8; SALTBYTES];
    fill_bytes(&mut salt);

    let key: [u8; 32] = derive_key(key, &salt)?;
    
    let nonce = Nonce::generate();

    let mut encrypted_data = encrypt(
        data.as_ref(),
        None,
        &nonce,
        &Key::from_bytes(key.as_ref())?,
    )?;

    encrypted_data.extend_from_slice(nonce.as_bytes());
    
    Ok((encrypted_data, salt))
}

pub fn derive_key(key : &[u8], salt : &[u8; 16]) -> Result<[u8; 32], ServerError> {
    let key: [u8; 32] = crypto_pwhash::pwhash(
        32,
        key.as_ref(),
        salt,
        crypto_pwhash::OPSLIMIT_INTERACTIVE,
        crypto_pwhash::MEMLIMIT_INTERACTIVE,
        crypto_pwhash::ALG_DEFAULT
    )?.as_slice()[..32].try_into().expect("slice with incorrect length");

    Ok(key)
}

pub fn decrypt_data<T: AsRef<[u8]>>(crypted_data: T, salt: &[u8; 16], key : T) -> Result<String, ServerError> {
    let key = derive_key(key.as_ref(), &salt)?;
    let crypted_data = crypted_data.as_ref();

    if crypted_data.len() <= NPUBBYTES + SALTBYTES {
        return Err(ServerError::UnknownError("Invalid length".to_owned()))
    }
    
    let (crypted_data, nonce) = crypted_data.split_at(crypted_data.len() - NPUBBYTES);
    
    let decrypted_data = decrypt(
        &crypted_data,
        None,
        &Nonce::from_bytes(nonce.try_into().expect("slice with incorrect length")),
        &Key::from_bytes(&key)?
    )?;
    
    Ok(String::from_utf8(decrypted_data).map_err(|_| ServerError::UnknownError("Invalid UTF-8".into()))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt() {
        const KEY: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let (crypted, salt) = encrypt_data(b"Some important data", &KEY).expect("failed to encrypt data");
        assert_eq!(decrypt_data(crypted, &salt, KEY.to_vec()).expect("failed to decrypt data"), "Some important data");
    }
}