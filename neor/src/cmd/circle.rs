use ql2::term::TermType;

use crate::arguments::{Args, CircleOption};
use crate::types::Point;
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

// TODO write test
