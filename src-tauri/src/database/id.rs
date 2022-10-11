use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, PartialEq, Eq)]
pub struct Id(String);

impl Default for Id {
    fn default() -> Self {
        Id(Uuid::new_v4().simple().to_string())
    }
}

impl Clone for Id {
    fn clone(&self) -> Self {
        Id(self.0.clone())
    }
}

impl From<String> for Id {
    fn from(string: String) -> Self {
        Id(string)
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0
    }
}
