use reql_rust::arguments::ReadMode;
use reql_rust::cmd::run::RunOption;
use reql_rust::{args, r, Result};

#[tokio::test]
async fn test_run_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let run_option = RunOption::default().read_mode(ReadMode::Outdated);
    let response = r
        .db("rethinkdb")
        .table("users")
        .run(args!(&conn, run_option))
        .await?;

    assert!(response.is_some());

    Ok(())
}
