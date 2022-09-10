use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Ungroup)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::types::UngroupItem;
    use crate::Result;

    #[tokio::test]
    async fn test_ungroup_data() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: Vec<UngroupItem<String, Post>> = table
            .group("title")
            .ungroup()
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained.len() == 4);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
