use cocoon::Cocoon;
use serde::Serialize;
use std::{env, fs::File, path::PathBuf, string::FromUtf8Error};

#[derive(thiserror::Error, Debug)]
pub enum WriteRefreshTokenError {
    #[error(transparent)]
    VarError(#[from] env::VarError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl Serialize for WriteRefreshTokenError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReadRefreshTokenError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("cocoon error")]
    CocoonError,
}

impl Serialize for ReadRefreshTokenError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub fn get_encryption_key() -> String {
    env::var("TOKEN_ENCRYPTION_KEY").unwrap()
}

pub fn get_authentication_file(bundle_identifier: String) -> PathBuf {
    tauri::api::path::local_data_dir()
        .unwrap()
        .join(bundle_identifier)
        .join("authentication")
}

pub fn write(
    refresh_token: String,
    bundle_identifier: String,
) -> Result<(), WriteRefreshTokenError> {
    tokio::spawn(async move {
        let data = refresh_token.as_bytes();
        let encryption_key = get_encryption_key();
        let cocoon = Cocoon::new(encryption_key.as_bytes());
        let resolved_path = get_authentication_file(bundle_identifier.clone());

        File::create(&resolved_path)
            .map(|mut file| cocoon.dump(data.to_vec(), &mut file).unwrap())
            .unwrap();
    });

    Ok(())
}

pub fn read(bundle_identifier: String) -> Result<String, ReadRefreshTokenError> {
    let resolved_path = get_authentication_file(bundle_identifier);
    let encryption_key = get_encryption_key();
    let cocoon = Cocoon::new(encryption_key.as_bytes());
    let mut file = File::open(&resolved_path)?;
    let bits = cocoon
        .parse(&mut file)
        .map_err(|_| ReadRefreshTokenError::CocoonError)?;
    Ok(String::from_utf8(bits)?)
}
