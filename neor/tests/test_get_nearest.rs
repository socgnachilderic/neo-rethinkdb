use neor::arguments::IndexCreateOption;
use neor::types::{ClosestDocumentResponse, Point};
use neor::{args, r, Converter, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
        .index_create(args!("area", IndexCreateOption::default().geo(true)))
        .run(&conn)
        .await?;
    table.index_wait(()).run(&conn).await?;
    table.insert(&data).run(&conn).await?;

    let secret_base = r.point(-122.422876, 37.777128);
    let response: Vec<ClosestDocumentResponse<Park>> = table
        .get_nearest(args!(secret_base, "area"))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.len() > 0);

    r.table_drop(table_name.as_str()).run(&conn).await?;
    Ok(())
}
