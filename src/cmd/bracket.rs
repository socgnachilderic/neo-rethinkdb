use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(attr: impl Serialize) -> Command {
    let arg = Command::from_json(attr);

    Command::new(TermType::Bracket).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_bracket_data() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: String = table
            .get(1)
            .bracket("title")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data.title);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
