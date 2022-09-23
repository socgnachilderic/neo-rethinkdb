use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new<S, T>(values: T) -> Command
where
    S: Serialize,
    T: IntoIterator<Item = S>,
{
    let mut command = Command::new(TermType::Object);

    for value in values {
        let arg = Command::from_json(value);

        command = command.with_arg(arg);
    }

    command
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::{r, Result};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct InnerPost {
        id: String,
        title: String,
    }

    #[tokio::test]
    async fn test_object_converted() -> Result<()> {
        let post = InnerPost {
            id: "id1".to_string(),
            title: "title1".to_string(),
        };
        let object = vec!["id", "id1", "title", "title1"];

        let conn = r.connection().connect().await?;
        let data_obtained: InnerPost = r.object(object).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == post);

        Ok(())
    }
}
