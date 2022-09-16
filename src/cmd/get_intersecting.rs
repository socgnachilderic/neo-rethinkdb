use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::prelude::Geometry;
use crate::Command;

pub(crate) fn new(args: impl GetIntersectingArg, index: &'static str) -> Command {
    let opts = GetIntersectingOption::default().index(index);

    Command::new(TermType::GetIntersecting)
        .with_arg(args.into_get_intersecting_opts())
        .with_opts(opts)
}

pub trait GetIntersectingArg {
    fn into_get_intersecting_opts(self) -> Command;
}

impl GetIntersectingArg for Command {
    fn into_get_intersecting_opts(self) -> Command {
        self
    }
}

impl<T> GetIntersectingArg for T
where
    T: Geometry,
{
    fn into_get_intersecting_opts(self) -> Command {
        self.get_command()
    }
}

#[derive(Debug, Clone, Serialize, Default, CommandOptions)]
pub struct GetIntersectingOption {
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::cmd::index_create::IndexCreateOption;
    use crate::cmd::point::Point;
    use crate::cmd::polygon::Polygon;
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Park {
        id: u8,
        area: Point,
    }

    impl Park {
        fn new(id: u8, area: Point) -> Self {
            Self { id, area }
        }
    }

    #[tokio::test]
    async fn test_get_intersecting_ops() -> Result<()> {
        let data = vec![
            Park::new(1, r.point(-0.1, 5.3)),
            Park::new(2, r.point(-117.220406, 32.719464)),
            Park::new(2, r.point(-120.6, 58.9)),
            Park::new(3, r.point(-11.220, 25.764)),
        ];
        let table_name = Uuid::new_v4().to_string();
        let conn = r.connection().connect().await?;
        let table = r.table(table_name.as_str());
        let circle: Polygon = r
            .circle(args!(r.point(-117.220406, 32.719464), 10.))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;
        r.table_create(table_name.as_str()).run(&conn).await?;
        table
            .clone()
            .index_create(("area", IndexCreateOption::default().geo(true)))
            .run(&conn)
            .await?;
        table.clone().index_wait(()).run(&conn).await?;
        table.clone().insert(&data).run(&conn).await?;

        let response: Vec<Park> = table
            .get_intersecting(circle, "area")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.len() == 1);

        r.table_drop(table_name.as_str()).run(&conn).await?;
        Ok(())
    }
}
