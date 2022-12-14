use neor::types::DbResponse;
use neor::{r, Converter, Result};
use uuid::Uuid;

#[tokio::test]
async fn test_create_db() -> Result<()> {
    let dbname = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;
    let db_created: DbResponse = r
        .db_create(dbname.as_str())
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(db_created.dbs_created == Some(1));

    r.db_drop(&dbname).run(&conn).await?;
    Ok(())
}
