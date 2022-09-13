use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(index: isize) -> Command {
    let arg = Command::from_json(index);

    Command::new(TermType::Nth).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::cmd::order_by::OrderByOption;
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_nth_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: Post = table
            .order_by(OrderByOption::default().index("title"))
            .nth(-1)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data.last() == Some(&data_obtained));

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
