use neor::arguments::{DeleteOption, ReturnChanges};
use neor::types::MutationResponse;
use neor::{Converter, Result};

use common::*;

mod common;

#[tokio::test]
async fn test_delete_docs() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: MutationResponse = table.get(5).delete(()).run(&conn).await?.unwrap().parse()?;

    assert!(response.deleted == 1);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_delete_docs_with_opts() -> Result<()> {
    let data = Post::get_many_data().get(0).unwrap().to_owned();
    let delete_option = DeleteOption::default().return_changes(ReturnChanges::Bool(true));
    let (conn, table, table_name) = set_up(true).await?;
    let response: MutationResponse = table
        .get(1)
        .delete(delete_option)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.deleted == 1);

    let old_val: Post = response
        .changes
        .unwrap()
        .first()
        .unwrap()
        .to_owned()
        .old_val
        .unwrap()
        .parse()?;

    assert!(old_val == data);

    tear_down(conn, &table_name).await
}
