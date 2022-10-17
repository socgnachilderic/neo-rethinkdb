use neor::types::DbResponse;
use neor::{r, Converter, Result};

#[tokio::test]
async fn test_drop_db() -> Result<()> {
    let dbname = "zuma";
    let conn = r.connection().connect().await?;
    r.db_create(dbname).run(&conn).await?;

    let db_dropped: DbResponse = r.db_drop(dbname).run(&conn).await?.unwrap().parse()?;

    assert!(db_dropped.dbs_dropped == Some(1));
    Ok(())
}
