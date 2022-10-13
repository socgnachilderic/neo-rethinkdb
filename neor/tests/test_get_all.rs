use neor::arguments::GetAllOption;
use neor::{args, Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_get_all() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;

    table.sync().run(&conn).await?;

    let data_get: Vec<Post> = table
        .get_all(args!(["title4"], GetAllOption::default().index("title")))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_get.len() == 2);
    assert!(data_get.first() == data.get(3));
    assert!(data_get.last() == data.last());

    tear_down(conn, &table_name).await
}
