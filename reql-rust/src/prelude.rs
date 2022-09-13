pub use futures::stream::select_all;
pub use futures::TryStreamExt;
#[doc(hidden)]
pub use reql_rust_macros::func;
pub use reql_rust_macros::Geometry;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Command;
pub use crate::cmd::func::Func;
pub use crate::cmd::StaticString;
pub use crate::Result;

pub trait Converter {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(self) -> Result<T>;
}

impl Converter for serde_json::Value {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_value(self)?)
    }
}

pub trait Geometry: Into<Command> {
    fn get_command(self) -> Command {
        self.into()
    }
}
