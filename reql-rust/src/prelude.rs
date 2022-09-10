pub use futures::stream::select_all;
pub use futures::TryStreamExt;
#[doc(hidden)]
pub use reql_rust_macros::func;
use serde::de::DeserializeOwned;
use serde::Serialize;

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

// pub trait Document: Serialize {
//     fn get_document(&self) -> &Self {
//         self
//     }
// }
