use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::types::Point;
use crate::Command;

#[derive(Debug, Clone)]
pub struct PointBuilder(pub(crate) Command, Point);

impl PointBuilder {
    pub(crate) fn new(point: Point) -> Self {
        let mut command = Command::new(TermType::Point);

        for coord in point.coordinates {
            let arg = Command::from_json(coord);
            command = command.with_arg(arg);
        }

        Self(command, point)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Point>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Point>> {
        self.0.into_arg::<()>().into_cmd().run::<_, Point>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl Serialize for PointBuilder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.1.serialize(serializer)
    }
}
