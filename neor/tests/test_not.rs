use std::ops::Not;

use neor::{r, Converter, Result};

#[tokio::test]
async fn test_not_data_r() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data_obtained: bool = r.not(r.expr(false)).run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained);

    Ok(())
}

#[tokio::test]
async fn test_not_data() -> Result<()> {
    let object = vec!["id", "id1", "title", "title1"];
    let conn = r.connection().connect().await?;
    let data_obtained: bool = r
        .object(object)
        .has_fields("content")
        .not()
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained);

    Ok(())
}
