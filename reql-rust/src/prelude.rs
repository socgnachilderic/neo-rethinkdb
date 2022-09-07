pub use crate::cmd::func::Func;
pub use crate::cmd::StaticString;
// pub use crate::ops::*;
pub use futures::stream::select_all;
pub use futures::TryStreamExt;
#[doc(hidden)]
pub use reql_rust_macros::func;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Converter {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(&self) -> T;
}

impl Converter for serde_json::Value {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(&self) -> T {
        serde_json::from_value(self.clone()).unwrap()
    }
}
