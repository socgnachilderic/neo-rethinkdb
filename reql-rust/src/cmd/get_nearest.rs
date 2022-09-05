use std::borrow::Cow;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::ops::ReqlOps;
use crate::types::{GeoSystem, Point, Unit};
use crate::Command;

use super::StaticString;

#[derive(Debug, Clone)]
pub struct GetNearestBuilder(pub(crate) Command, pub(crate) GetNearestOption);

#[derive(Debug, Clone, Serialize, Default)]
#[non_exhaustive]
pub struct GetNearestOption {
    pub index: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dist: Option<usize>,
}

impl GetNearestBuilder {
    pub(crate) fn new(point: &Point, index: &'static str) -> Self {
        let arg = Command::from_json(point);
        let command = Command::new(TermType::GetNearest).with_arg(arg);
        let opts = GetNearestOption {
            index: index.static_string(),
            ..Default::default()
        };

        Self(command, opts)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<serde_json::Value>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<serde_json::Value>> {
        self.get_parent().run::<_, serde_json::Value>(arg)
    }

    pub fn with_geo_system(mut self, geo_system: GeoSystem) -> Self {
        self.1.geo_system = Some(geo_system);
        self
    }

    pub fn with_unit(mut self, unit: Unit) -> Self {
        self.1.unit = Some(unit);
        self
    }

    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.1.max_results = Some(max_results);
        self
    }

    pub fn with_max_dist(mut self, max_dist: usize) -> Self {
        self.1.max_dist = Some(max_dist);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for GetNearestBuilder {
    fn get_parent(&self) -> Command {
        self.0
            .clone()
            .with_opts(&self.1)
            .into_arg::<()>()
            .into_cmd()
    }
}

impl Into<Command> for GetNearestBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
