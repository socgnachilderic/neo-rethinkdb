use neor::types::Time;
use neor::{r, Converter, Result};

#[tokio::test]
async fn test_date_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let datetime = r.now().date();
    let date1 = datetime.value();
    let date2: Time = datetime.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(date1 == date2);

    Ok(())
}
