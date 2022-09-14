use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(step: usize) -> Command {
    let arg = Command::from_json(step);

    Command::new(TermType::Skip).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post,};
    use crate::Result;

    #[tokio::test]
    async fn test_skip_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up( true).await?;
        let data_obtained: Vec<Post> = table
            .skip(data.len() - 1)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained.first() == data.first());

        tear_down(conn, &table_name).await
    }
}
