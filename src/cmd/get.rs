use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl GetArg) -> Command {
    Command::new(TermType::Get).with_arg(args.into_get_opts())
}

pub trait GetArg {
    fn into_get_opts(self) -> Command;
}

impl GetArg for Command {
    fn into_get_opts(self) -> Command {
        self
    }
}

impl<T> GetArg for T
where
    T: Serialize,
{
    fn into_get_opts(self) -> Command {
        Command::from_json(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[tokio::test]
    async fn test_get_data() -> Result<()> {
        let expected_post = Post::get_many_data().get(3).unwrap().to_owned();
        let (conn, table, table_name) = set_up(true).await?;
        let data_inserted: Option<Post> = table
            .get(expected_post.id)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted == Some(expected_post));

        tear_down(conn, table_name.as_str()).await
    }
}
