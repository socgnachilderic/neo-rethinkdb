use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, de::DeserializeOwned};

use crate::ops::{ReqlOps, ReqlOpsGeometry};
use crate::types::{GeoSystem, Point, Unit};
use crate::Command;

#[derive(Debug, Clone)]
pub struct CircleBuilder<T>(
    pub(crate) Command,
    pub(crate) CircleOption,
    pub(crate) PhantomData<T>,
);

#[derive(Debug, Clone, Serialize, Default)]
#[non_exhaustive]
pub struct CircleOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_vertices: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
}

impl<T: Unpin + DeserializeOwned + Serialize + ReqlOpsGeometry> CircleBuilder<T> {
    pub(crate) fn new(point: &Point, radius: u32) -> Self {
        let arg_point = Command::from_json(point);
        let arg_radius = Command::from_json(radius);
        let command = Command::new(TermType::Circle)
            .with_arg(arg_point)
            .with_arg(arg_radius);

        Self(command, CircleOption::default(), PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<T>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, T>(arg)
    }

    pub fn with_geo_system(mut self, geo_system: GeoSystem) -> Self {
        self.1.geo_system = Some(geo_system);
        self
    }

    pub fn with_unit(mut self, unit: Unit) -> Self {
        self.1.unit = Some(unit);
        self
    }

    pub fn with_num_vertices(mut self, num_vertices: usize) -> Self {
        self.1.num_vertices = Some(num_vertices);
        self
    }

    pub(crate) fn with_fill(mut self, fill: bool) -> Self {
        self.1.fill = Some(fill);
        self
    }
}

impl<T> ReqlOpsGeometry for CircleBuilder<T> {}

impl<T> ReqlOps for CircleBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
