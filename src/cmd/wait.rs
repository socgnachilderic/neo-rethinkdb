use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Args, WaitFor};
use crate::Command;

pub(crate) fn new(args: impl WaitArg) -> Command {
    let (args, opts) = args.into_wait_opts();
    let mut command = Command::new(TermType::Wait);

    if let Some(arg) = args {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait WaitArg {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption);
}

impl WaitArg for () {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        Default::default()
    }
}

impl WaitArg for WaitOption {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        (None, self)
    }
}

impl WaitArg for Command {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        (Some(self), Default::default())
    }
}

impl WaitArg for Args<(Command, WaitOption)> {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        (Some(self.0 .0), self.0 .1)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct WaitOption {
    /// a enum indicating a table status to wait on before returning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<WaitFor>,
    /// a number indicating maximum time, in seconds,
    /// to wait for the table to be ready.
    /// If this value is exceeded, a ReqlRuntimeError will be thrown.
    /// A value of0 means no timeout. The default is 0 (no timeout).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::WaitResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_wait_table() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: WaitResponse = table.wait(()).run(&conn).await?.unwrap().parse()?;

        assert!(response.ready == 1);

        tear_down(conn, &table_name).await
    }
}
