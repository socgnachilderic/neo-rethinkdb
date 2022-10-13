use neor::arguments::IndexRenameOption;
use neor::types::IndexResponse;
use neor::{args, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_rename_index() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    table.index_create("author").run(&conn).await?;
    let index_renamed: IndexResponse = table
        .clone()
        .index_rename(args!("author", "author_name"))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(index_renamed.renamed > Some(0));

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_rename_index_with_overwrite() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    table.index_create("author").run(&conn).await?;
    table.index_create("author_name").run(&conn).await?;

    let index_renamed: IndexResponse = table
        .clone()
        .index_rename(args!(
            "author",
            "author_name",
            IndexRenameOption::default().overwrite(true)
        ))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(index_renamed.renamed > Some(0));

    tear_down(conn, &table_name).await
}
