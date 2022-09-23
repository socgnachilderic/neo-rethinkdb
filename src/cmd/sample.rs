use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(number: usize) -> Command {
    let arg = Command::from_json(number);

    Command::new(TermType::Sample).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[tokio::test]
    async fn test_sample_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: Vec<Post> = table.sample(3).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained.len() == 3);

        tear_down(conn, &table_name).await
    }
}
