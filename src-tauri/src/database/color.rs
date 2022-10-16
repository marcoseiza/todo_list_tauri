use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, Default, Clone, Copy, EnumString, Deserialize)]
pub enum Color {
    #[default]
    BLUE,
    GREEN,
    RED,
    ORANGE,
    YELLOW,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&*self.to_string())
    }
}
