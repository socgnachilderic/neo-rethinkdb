use ql2::term::TermType;

use crate::types::Polygon;
use crate::Command;

pub(crate) fn new(polygon: Polygon) -> Command {
    let arg: Command = polygon.into();

    Command::new(TermType::PolygonSub).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::cmd::polygon::Polygon;
    use crate::prelude::Converter;
    use crate::types::Point;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_polygon_sub_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let expected_data = Polygon::new_from_vec(vec![
            vec![
                [-122.4, 37.7],
                [-122.4, 37.3],
                [-121.8, 37.3],
                [-121.8, 37.7],
                [-122.4, 37.7],
            ],
            vec![
                [-122.3, 37.4],
                [-122.3, 37.6],
                [-122.0, 37.6],
                [-122.0, 37.4],
                [-122.3, 37.4],
            ],
        ]);
        let outer_polygon = r.polygon(&[
            Point::new(-122.4, 37.7),
            Point::new(-122.4, 37.3),
            Point::new(-121.8, 37.3),
            Point::new(-121.8, 37.7),
        ]);
        let inner_polygon = r.polygon(&[
            Point::new(-122.3, 37.4),
            Point::new(-122.3, 37.6),
            Point::new(-122.0, 37.6),
            Point::new(-122.0, 37.4),
        ]);
        let response: Polygon = outer_polygon
            .polygon_sub(inner_polygon)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == expected_data);

        Ok(())
    }
}
