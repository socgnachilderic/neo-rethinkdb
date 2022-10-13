use neor::arguments::Permission;
use neor::types::{ConfigChange, GrantChangeValue, GrantResponse};
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

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
    let response: GrantResponse = table
        .grant("malik", permissions)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == expected);

    tear_down(conn, &table_name).await
}
