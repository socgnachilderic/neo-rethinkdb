use ql2::term::TermType;

use crate::arguments::Permission;
use crate::Command;

pub(crate) fn new(username: &str, permission: Permission) -> Command {
    Command::new(TermType::Grant)
        .with_arg(Command::from_json(username))
        .with_arg(Command::from_json(permission))
}

#[cfg(test)]
mod tests {
    use crate::arguments::Permission;
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::{ConfigChange, GrantChangeValue, GrantResponse};
    use crate::Result;

    #[tokio::test]
    async fn test_grant_permission() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
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
        let permissions = Permission::default().read(true).write(true);
        // TODO Replace current user when test user should be created
        let response: GrantResponse = table
            .grant("bob", permissions)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == expected);

        tear_down(conn, &table_name).await
    }
}
