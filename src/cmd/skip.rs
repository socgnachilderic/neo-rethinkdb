use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(step: usize) -> Command {
    let arg = Command::from_json(step);

    Command::new(TermType::Skip).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_skip_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: Vec<Post> = table
            .skip(data.len() - 1)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained.first() == data.first());

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
