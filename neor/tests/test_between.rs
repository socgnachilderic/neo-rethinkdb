use neor::arguments::{BetweenOption, Status};
use neor::{args, r, Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_get_data_between() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_get: Vec<Post> = table
        .between(args!(2, 4))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_get.len() == 2);
    assert!(data_get.first() == data.get(2));
    assert!(data_get.last() == data.get(1));

    tear_down(conn, table_name.as_str()).await
}

#[tokio::test]
async fn test_get_data_between_by_minval() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_get: Vec<Post> = table
        .between(args!(r::min_val(), 4))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_get.len() == 3);
    assert!(data_get.first() == data.get(2));
    assert!(data_get.last() == data.first());

    tear_down(conn, table_name.as_str()).await
}

#[tokio::test]
async fn test_get_data_between_by_maxval() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_get: Vec<Post> = table
        .between(args!(2, r::max_val()))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_get.len() == 4);
    assert!(data_get.first() == data.get(3));
    assert!(data_get.last() == data.get(1));

    tear_down(conn, table_name.as_str()).await
}

#[tokio::test]
async fn test_get_data_between_with_opts() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let between_option = BetweenOption::default().right_bound(Status::Closed);
    let data_get: Vec<Post> = table
        .between(args!(r.expr(2), r.expr(4), between_option))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_get.len() == 3);
    assert!(data_get.first() == data.get(3));
    assert!(data_get.last() == data.get(1));

    tear_down(conn, table_name.as_str()).await
}

#[tokio::test]
async fn test_get_data_between_by_minval_and_max_val_with_opts() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let between_option = BetweenOption::default()
        .right_bound(Status::Closed)
        .left_bound(Status::Closed)
        .index("title");
    let data_get: Vec<Post> = table
        .between(args!(r::min_val(), r::max_val(), between_option))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_get.len() == data.len());
    assert!(data_get.first() == data.get(3));
    assert!(data_get.last() == data.first());

    tear_down(conn, table_name.as_str()).await
}
