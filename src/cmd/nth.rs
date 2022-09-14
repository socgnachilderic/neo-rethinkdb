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
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[tokio::test]
    async fn test_nth_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: Post = table
            .order_by(OrderByOption::default().index("title"))
            .nth(-1)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data.last() == Some(&data_obtained));

        tear_down(conn, &table_name).await
    }
}
