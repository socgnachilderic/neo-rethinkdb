use futures::stream::{select_all, TryStreamExt};
use reql_rust::arguments::ReadMode;
use reql_rust::cmd::run::RunOption;
use reql_rust::prelude::Converter;
use reql_rust::{args, r, Result};

#[tokio::test]
async fn simple() -> Result<()> {
    let conn = r.connection().connect().await?;
    let run_option = RunOption::default().read_mode(ReadMode::Outdated);
    let response = r
        .db("rethinkdb")
        .table("users")
        .make_query(args!(&conn, run_option))
        .try_next()
        .await?;

    assert!(response.is_some());

    Ok(())
}

#[tokio::test]
async fn concurrency() -> Result<()> {
    let conn = r.connection().connect().await?;
    let expected_messages: Vec<String> = (0..10_000)
        .into_iter()
        .map(|i| format!("message {}", i))
        .collect();

    let mut streams = Vec::new();

    for msg in expected_messages.iter() {
        streams.push(r.expr(msg).make_query(&conn));
    }

    let mut list = select_all(streams);
    let mut response = Vec::new();

    while let Some(msg) = list.try_next().await? {
        let msg: String = msg.parse()?;
        response.push(msg);
    }

    assert!(response == expected_messages);

    Ok(())
}
