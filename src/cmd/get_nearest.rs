use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::{
    prelude::Geometry,
    types::{GeoSystem, Unit},
    Command,
};

pub(crate) fn new(args: impl GetNearestArg) -> Command {
    let (arg, opts) = args.into_get_nearest_opts();

    Command::new(TermType::GetNearest)
        .with_arg(arg)
        .with_opts(opts)
}

pub trait GetNearestArg {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption);
}

impl<T: Geometry> GetNearestArg for (T, &str) {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption) {
        let index_name = GetNearestOption::default().index(self.1.to_owned());

        (self.0.into(), index_name)
    }
}

impl<T: Geometry> GetNearestArg for (T, &str, GetNearestOption) {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption) {
        let index_name = self.2.index(self.1.to_owned());

        (self.0.into(), index_name)
    }
}

#[derive(Debug, Clone, Serialize, Default, CommandOptions)]
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

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::cmd::index_create::IndexCreateOption;
    use crate::cmd::point::Point;
    use crate::prelude::Converter;
    use crate::types::ClosestDocumentResponse;
    use crate::{r, Result};

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
    async fn test_get_nearest_ops() -> Result<()> {
        let data = vec![
            Park::new(1, r.point(-121.886420, 37.329898)),
            Park::new(2, r.point(-117.220406, 32.719464)),
            Park::new(3, r.point(-122.422876, 37.777128)),
            Park::new(4, r.point(-122.423246, 37.779388)),
        ];
        let table_name = Uuid::new_v4().to_string();
        let conn = r.connection().connect().await?;
        let table = r.table(table_name.as_str());
        r.table_create(table_name.as_str()).run(&conn).await?;
        table
            .clone()
            .index_create(("area", IndexCreateOption::default().geo(true)))
            .run(&conn)
            .await?;
        table.clone().index_wait(()).run(&conn).await?;
        table.clone().insert(&data).run(&conn).await?;

        let secret_base = r.point(-122.422876, 37.777128);
        let response: Vec<ClosestDocumentResponse<Park>> = table
            .get_nearest((secret_base, "area"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.len() > 0);

        r.table_drop(table_name.as_str()).run(&conn).await?;
        Ok(())
    }
}