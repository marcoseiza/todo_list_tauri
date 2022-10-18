use serde::{Deserialize, Serialize};

use crate::database::crypto::Secure;
use crate::helpers::{join_bytes, separate_bytes};
use crate::{database::id::Id, helpers::trim_whitespace};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Task {
    pub id: Id,
    pub body: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct EncryptedTask {
    pub id: Id,
    pub body: String,
}

impl Task {
    pub fn default_from(body: String) -> Self {
        Task {
            body: trim_whitespace(body.trim()),
            ..Default::default()
        }
    }
}

impl Secure for Task {
    type Encrypted = EncryptedTask;

    fn encrypt(&self, cocoon: &cocoon::Cocoon<cocoon::Creation>) -> Self::Encrypted {
        Self::Encrypted {
            id: self.id.clone(),
            body: join_bytes(&cocoon.wrap(self.body.as_bytes()).unwrap(), ","),
        }
    }

    fn decrypt(cocoon: &cocoon::Cocoon<cocoon::Creation>, container: &Self::Encrypted) -> Self {
        Self {
            id: container.id.clone(),
            body: String::from_utf8(
                cocoon
                    .unwrap(&separate_bytes(&container.body, ","))
                    .unwrap(),
            )
            .unwrap(),
        }
    }
}
