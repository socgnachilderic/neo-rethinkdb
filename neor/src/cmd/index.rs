use serde::Serialize;

use crate::proto::Query;
use crate::Command;

pub fn new(args: impl IndexArg) -> Index {
    args.into_index_opts()
}

#[derive(Debug, Clone)]
pub struct Index(pub(crate) Command);

#[derive(Serialize)]
struct Inner<'a> {
    index: Query<'a>,
}

pub trait IndexArg {
    fn into_index_opts(self) -> Index;
}

impl<T> IndexArg for T
where
    T: Into<String>,
{
    fn into_index_opts(self) -> Index {
        Index(Command::from_json(self.into()))
    }
}

impl IndexArg for Command {
    fn into_index_opts(self) -> Index {
        Index(self)
    }
}
