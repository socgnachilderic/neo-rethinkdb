use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::ops::ReqlOpsGeometry;
use crate::types::{GeoSystem, Unit};
use crate::Command;

#[derive(Debug, Clone)]
pub struct DistanceBuilder(pub(crate) Command, pub(crate) DistanceOption);

#[derive(Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DistanceOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
}

impl DistanceBuilder {
    pub(crate) fn new<A: ReqlOpsGeometry + Serialize>(geometry: A) -> Self {
        let arg = Command::from_json(geometry);
        let command = Command::new(TermType::Distance).with_arg(arg);

        Self(command, DistanceOption::default())
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<usize>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<usize>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, usize>(arg)
    }

    pub fn with_geo_system(mut self, geo_system: GeoSystem) -> Self {
        self.1.geo_system = Some(geo_system);
        self
    }

    pub fn with_unit(mut self, unit: Unit) -> Self {
        self.1.unit = Some(unit);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
