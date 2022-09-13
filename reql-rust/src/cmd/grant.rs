use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl GrantArg) -> Command {
    let (arg, opts) = args.into_grant_opts();

    Command::new(TermType::Grant)
        .with_arg(arg)
        .with_arg(Command::from_json(opts))
}

pub trait GrantArg {
    fn into_grant_opts(self) -> (Command, GrantOption);
}

impl GrantArg for &str {
    fn into_grant_opts(self) -> (Command, GrantOption) {
        (Command::from_json(self), Default::default())
    }
}

impl GrantArg for (&str, GrantOption) {
    fn into_grant_opts(self) -> (Command, GrantOption) {
        (Command::from_json(self.0), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct GrantOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::{ConfigChange, GrantChangeValue, GrantResponse};
    use crate::Result;

    use super::GrantOption;

    #[tokio::test]
    async fn test_grant_permission() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], false).await?;
        let expected = GrantResponse {
            granted: 1,
            permissions_changes: vec![ConfigChange {
                old_val: None,
                new_val: Some(GrantChangeValue {
                    write: Some(true),
                    read: Some(true),
                    config: None,
                    connect: None,
                }),
            }],
        };
        let permissions = GrantOption::default().read(true).write(true);
        // TODO Replace current user when test user should be created
        let response: GrantResponse = table
            .grant(("bob", permissions))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == expected);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
