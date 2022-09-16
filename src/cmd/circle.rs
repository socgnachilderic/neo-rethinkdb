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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_vertices: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
}

// TODO write test
