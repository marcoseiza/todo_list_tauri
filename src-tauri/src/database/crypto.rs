use serde::{de::DeserializeOwned, Serialize};

pub trait Secure {
    type Encrypted: Serialize + DeserializeOwned;

    fn encrypt(&self, cocoon: &cocoon::Cocoon<cocoon::Creation>) -> Self::Encrypted;

    fn decrypt(cocoon: &cocoon::Cocoon<cocoon::Creation>, container: &Self::Encrypted) -> Self;
}
