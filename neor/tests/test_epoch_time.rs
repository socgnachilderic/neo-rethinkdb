use neor::types::Time;
use neor::{r, Converter, Result};

#[tokio::test]
async fn test_time_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let date_time = r.epoch_time(531360000)?;
    let time1 = date_time.clone().value();
    let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(time2 == time1);

    Ok(())
}
