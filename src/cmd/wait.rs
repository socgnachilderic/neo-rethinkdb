use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::types::WaitFor;
use crate::Command;

pub(crate) fn new(args: impl WaitArg) -> Command {
    Command::new(TermType::Wait).with_opts(args.into_wait_opts())
}

pub trait WaitArg {
    fn into_wait_opts(self) -> WaitOption;
}

impl WaitArg for () {
    fn into_wait_opts(self) -> WaitOption {
        Default::default()
    }
}

impl WaitArg for WaitOption {
    fn into_wait_opts(self) -> WaitOption {
        self
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct WaitOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<WaitFor>,
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
