use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::Command;
use crate::types::GeoJson;

#[derive(Debug, Clone)]
pub struct ToGeoJsonBuilder<T>(pub(crate) Command, PhantomData<T>);

impl<T: Unpin + Serialize + DeserializeOwned + Clone> ToGeoJsonBuilder<T> {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::ToGeojson);

        Self(command, PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<GeoJson<T>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<GeoJson<T>>> {
        self.0.into_arg::<()>().into_cmd().run::<_, GeoJson<T>>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
