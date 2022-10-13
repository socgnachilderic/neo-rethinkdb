use ql2::term::TermType;

use crate::prelude::Func;
use crate::{types::Binary, Command};

pub(crate) fn new(args: impl SetWriteHookArg) -> Command {
    Command::new(TermType::SetWriteHook).with_arg(args.into_set_write_hook_opts())
}

pub trait SetWriteHookArg {
    fn into_set_write_hook_opts(self) -> Command;
}

impl SetWriteHookArg for Command {
    fn into_set_write_hook_opts(self) -> Command {
        self
    }
}

impl SetWriteHookArg for Option<u8> {
    fn into_set_write_hook_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl SetWriteHookArg for Func {
    fn into_set_write_hook_opts(self) -> Command {
        self.0
    }
}

impl SetWriteHookArg for Binary {
    fn into_set_write_hook_opts(self) -> Command {
        Command::from_json(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down};
    use crate::types::SetWriteHookResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_set_write_hook_ops() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;

        let response: SetWriteHookResponse = table
            .set_write_hook(func!(|_, _, new_val| new_val))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert_eq!(response.created, Some(1));

        tear_down(conn, &table_name).await
    }
}
