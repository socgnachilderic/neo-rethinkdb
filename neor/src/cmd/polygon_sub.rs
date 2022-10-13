use ql2::term::TermType;

use crate::types::Polygon;
use crate::Command;

pub(crate) fn new(polygon: Polygon) -> Command {
    let arg: Command = polygon.into();

    Command::new(TermType::PolygonSub).with_arg(arg)
}
