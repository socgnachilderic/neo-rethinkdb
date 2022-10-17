use neor::{r, Converter, Result};

#[tokio::test]
async fn test_div_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: f64 = (r.expr(2) / 2).run(&conn).await?.unwrap().parse()?;

    assert_eq!(response, 1.);

    Ok(())
}
