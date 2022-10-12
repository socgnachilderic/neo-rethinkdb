use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::GetWriteHook)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down};
    use crate::types::WriteHookResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_get_write_hook_ops() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        table
            .clone()
            .set_write_hook(func!(|_, _, new_val| new_val))
            .run(&conn)
            .await?;

        let response: WriteHookResponse =
            table.get_write_hook().run(&conn).await?.unwrap().parse()?;

        assert_eq!(
            response.query,
            "setWriteHook(function(var1, var2, var3) { return var3; })"
        );

        tear_down(conn, &table_name).await
    }
}
