use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, Default, Clone, Copy, EnumString)]
pub enum Color {
    #[default]
    #[strum(serialize = "bg-blue-700")]
    BLUE,
    #[strum(serialize = "bg-red-700")]
    RED,
    #[strum(serialize = "bg-green-700")]
    GREEN,
    #[strum(serialize = "bg-orange-700")]
    ORANGE,
    #[strum(serialize = "bg-yellow-600")]
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
