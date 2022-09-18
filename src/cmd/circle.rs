use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Args, Unit};
use crate::types::{GeoSystem, Point};
use crate::Command;

pub(crate) fn new(args: impl CircleArg) -> Command {
    let (arg_point, arg_radius, opts) = args.into_circle_opts();

    Command::new(TermType::Circle)
        .with_arg(arg_point)
        .with_arg(arg_radius)
        .with_opts(opts)
}

pub trait CircleArg {
    fn into_circle_opts(self) -> (Command, Command, CircleOption);
}

impl CircleArg for Args<(Point, f64)> {
    fn into_circle_opts(self) -> (Command, Command, CircleOption) {
        (
            Command::from_json(self.0 .0),
            Command::from_json(self.0 .1),
            Default::default(),
        )
    }
}

impl CircleArg for Args<(Command, f64)> {
    fn into_circle_opts(self) -> (Command, Command, CircleOption) {
        (self.0 .0, Command::from_json(self.0 .1), Default::default())
    }
}

impl CircleArg for Args<(Point, f64, CircleOption)> {
    fn into_circle_opts(self) -> (Command, Command, CircleOption) {
        (
            Command::from_json(self.0 .0),
            Command::from_json(self.0 .1),
            self.0 .2,
        )
    }
}

impl CircleArg for Args<(Command, f64, CircleOption)> {
    fn into_circle_opts(self) -> (Command, Command, CircleOption) {
        (self.0 .0, Command::from_json(self.0 .1), self.0 .2)
    }
}

#[derive(
    Debug, Clone, Serialize, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CommandOptions,
)]
pub struct CircleOption {
    /// the number of vertices in the polygon or line. Defaults to 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_vertices: Option<usize>,
    /// the reference ellipsoid to use for geographic coordinates.
    /// Possible values are `WGS84` (the default), a common standard
    /// for Earth’s geometry, or `UnitSphere`, a perfect sphere of 1 meter radius.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
    /// Unit for the radius distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// if `true` (the default) the circle is filled, creating a polygon;
    /// if `false` the circle is unfilled (creating a line).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
}

// TODO write test
