use std::ops::Not;

use neor::{r, Converter, Result};

#[tokio::test]
async fn test_not_data_r() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: bool = r.not(r.expr(false)).run(&conn).await?.unwrap().parse()?;

    assert!(response);

    Ok(())
}

#[tokio::test]
async fn test_not_data() -> Result<()> {
    let object = vec!["id", "id1", "title", "title1"];
    let conn = r.connection().connect().await?;
    let response: bool = r
        .object(object)
        .has_fields("content")
        .not()
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response);

    Ok(())
}
