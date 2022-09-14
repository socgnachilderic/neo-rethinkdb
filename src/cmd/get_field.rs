use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(attr: impl Serialize) -> Command {
    let arg = Command::from_json(attr);

    Command::new(TermType::GetField).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[tokio::test]
    async fn test_get_fields() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: String = table.get(1).g("title").run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == data.title);

        tear_down(conn, &table_name).await
    }
}
