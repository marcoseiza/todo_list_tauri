use cocoon::Cocoon;
use std::{fs::File, path::PathBuf};

use crate::helpers::stringify_error;

pub fn get_encryption_key() -> String {
    dotenv!("TOKEN_ENCRYPTION_KEY").to_string()
}

pub fn get_authentication_file(bundle_identifier: String) -> PathBuf {
    tauri::api::path::local_data_dir()
        .unwrap()
        .join(bundle_identifier)
        .join("authentication")
}

pub fn write(refresh_token: String, bundle_identifier: String) -> Result<(), String> {
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

pub fn read(bundle_identifier: String) -> Result<String, String> {
    let resolved_path = get_authentication_file(bundle_identifier);
    let encryption_key = get_encryption_key();
    let cocoon = Cocoon::new(encryption_key.as_bytes());
    let mut file = File::open(&resolved_path).map_err(stringify_error)?;
    let bits = cocoon
        .parse(&mut file)
        .map_err(|_| String::from("Error parsing with cocoon."))?;
    String::from_utf8(bits).map_err(stringify_error)
}
