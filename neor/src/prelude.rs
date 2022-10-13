pub use futures::stream::select_all;
pub use futures::TryStreamExt;
pub use neor_macros::Geometry;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use crate::cmd::StaticString;

pub trait Converter {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(self) -> Result<T>;
}

impl Converter for serde_json::Value {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_value(self)?)
    }
}

pub trait Geometry: Into<Command> {
    fn cmd(self) -> Command {
        self.into()
    }
}
