use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReturnChanges {
    Bool(bool),
    Always,
}

impl Serialize for ReturnChanges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Always => "always".serialize(serializer),
        }
    }
}
