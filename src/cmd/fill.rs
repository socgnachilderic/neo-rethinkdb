use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Fill)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::types::{Point, Polygon};
    use crate::{r, Result};

    #[tokio::test]
    async fn test_fill_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let rectangle = r.line(&[
            Point::new(-122.423246, 37.779388),
            Point::new(-122.423246, 37.329898),
            Point::new(-121.886420, 37.329898),
            Point::new(-121.886420, 37.779388),
        ]);
        let data = Polygon::new(&[
            Point::new(-122.423246, 37.779388),
            Point::new(-122.423246, 37.329898),
            Point::new(-121.88642, 37.329898),
            Point::new(-121.88642, 37.779388),
            Point::new(-122.423246, 37.779388),
        ]);

        let response: Polygon = rectangle.fill().run(&conn).await?.unwrap().parse()?;

        assert!(response == data);

        Ok(())
    }
}
