use neor::{r, Converter, Result};

#[tokio::test]
async fn test_day_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let day = r.now().day();
    let day1 = day.value();
    let day2: u8 = day.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(day1 == day2);

    Ok(())
}
