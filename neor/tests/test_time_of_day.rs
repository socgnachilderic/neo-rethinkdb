use neor::{r, Converter, Result};

#[tokio::test]
async fn test_date_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let datetime = r.now().time_of_day();
    let date1 = datetime.value();
    let date2: f64 = datetime.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(date1.is_normal());
    assert!(date2.is_normal());

    Ok(())
}
