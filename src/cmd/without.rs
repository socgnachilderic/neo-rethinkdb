use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(selector: impl Serialize) -> Command {
    let arg = Command::from_json(selector);

    Command::new(TermType::Without).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct InnerPost {
        id: u8,
        title: String,
    }

    #[tokio::test]
    async fn test_without_data() -> Result<()> {
        let data = Post::get_one_data();
        let data = InnerPost {
            id: data.id,
            title: data.title,
        };
        let (conn, table, table_name) = set_up( true).await?;
        let data_obtained: InnerPost = table
            .get(1)
            .without(["content", "view"])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data);

        tear_down(conn, &table_name).await
    }
}
