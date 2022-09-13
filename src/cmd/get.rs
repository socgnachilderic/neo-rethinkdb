use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(primary_key: impl Serialize) -> Command {
    let arg = Command::from_json(primary_key);

    Command::new(TermType::Get).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_get_data() -> Result<()> {
        let expected_post = Post::get_many_data().get(3).unwrap().to_owned();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_inserted: Option<Post> = table
            .get(expected_post.id)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted == Some(expected_post));

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
