use ql2::term::TermType;

use crate::types::AnyParam;
use crate::Command;

pub(crate) fn new(fields: AnyParam) -> Command {
    let arg: Command = fields.into();

    Command::new(TermType::WithFields).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::types::AnyParam;
    use crate::Result;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    struct InnerPost {
        id: u8,
        title: String,
    }

    #[tokio::test]
    async fn test_with_fields() -> Result<()> {
        let data: Vec<InnerPost> = Post::get_many_data()
            .into_iter()
            .map(|post| InnerPost {
                id: post.id,
                title: post.title,
            })
            .collect();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let mut data_obtained: Vec<InnerPost> = table
            .with_fields(AnyParam::new(["id", "title"]))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        data_obtained.sort_by(|a, b| a.id.cmp(&b.id));
        
        assert!(data_obtained == data);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
