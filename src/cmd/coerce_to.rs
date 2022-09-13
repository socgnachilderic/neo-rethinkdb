use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(value: impl Serialize) -> Command {
    Command::new(TermType::CoerceTo).with_arg(Command::from_json(value))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::prelude::Converter;
    use crate::spec::Post;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_coerce_to_ops() -> Result<()> {
        let data = Post::get_one_data();
        let conn = r.connection().connect().await?;
        let response: Post = r
            .expr(json!([
                ["id", 1],
                ["title", "title1"],
                ["content", "content1"],
                ["view", 0]
            ]))
            .coerce_to("object")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        let response2: char = r
            .expr(1)
            .coerce_to("string")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == data);
        assert!(response2 == '1');

        Ok(())
    }
}
