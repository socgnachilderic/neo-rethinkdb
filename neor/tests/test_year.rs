use neor::{r, Converter, Result};

#[tokio::test]
async fn test_year_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let year = r.now().year();
    let year1 = year.value();
    let year2: i32 = year.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(year1 == year2);

    Ok(())
}
