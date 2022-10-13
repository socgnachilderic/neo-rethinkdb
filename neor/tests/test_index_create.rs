use neor::arguments::IndexCreateOption;
use neor::types::IndexResponse;
use neor::{args, r, Command, Converter, Result, Session};
use uuid::Uuid;

#[tokio::test]
async fn test_create_index() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;
    let index_created = r.table(table_name.as_str()).index_create("author");

    setup(&table_name, index_created, &conn).await
}

#[tokio::test]
async fn test_create_index_with_options() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;
    let index_option = IndexCreateOption::default().multi(true);
    let index_created = r
        .table(table_name.as_str())
        .index_create(args!("author", index_option));

    setup(&table_name, index_created, &conn).await
}

/* #[tokio::test]
async fn test_create_index_with_func() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;
    let index_created = r
        .table(table_name.as_str())
        .index_create(("author", func!(|row| row.bracket("author").bracket("name"))));

    setup(&table_name, index_created, &conn).await
}

#[tokio::test]
async fn test_create_index_with_func_and_options() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;
    let index_option = IndexCreateOption::default().multi(true);
    let index_created = r.table(table_name.as_str()).index_create((
        "author",
        func!(|row| row.bracket("author").bracket("name")),
        index_option,
    ));

    setup(&table_name, index_created, &conn).await
} */

async fn setup(table_name: &str, index_created: Command, conn: &Session) -> Result<()> {
    r.table_create(table_name).run(conn).await?;

    let index_created: IndexResponse = index_created.run(conn).await?.unwrap().parse()?;

    assert!(index_created.created > Some(0));

    r.table_drop(table_name).run(conn).await?;
    Ok(())
}
