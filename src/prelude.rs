use std::sync::atomic::{AtomicU64, Ordering};

pub use futures::stream::select_all;
pub use futures::TryStreamExt;
#[doc(hidden)]
pub use reql_macros::func;
pub use reql_macros::Geometry;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use crate::cmd::func::Func;
pub use crate::cmd::StaticString;
pub use crate::proto::Command;
pub use crate::Result;

#[doc(hidden)]
pub static VAR_COUNTER: AtomicU64 = AtomicU64::new(1);

#[doc(hidden)]
pub fn var_counter() -> u64 {
    VAR_COUNTER.fetch_add(1, Ordering::SeqCst)
}

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

// fn foo() {
//     let var = args!(1, String::from("2"), GeoSystem::WGS84);
// }

// print!()
