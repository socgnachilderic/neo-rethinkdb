use neor::types::Time;
use neor::{r, Converter, Result};
use time::macros::offset;

#[tokio::test]
async fn test_in_timezone_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let date_time = r.now().in_timezone(offset!(-08:00));
    let time1 = date_time.value();
    let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(time1.is_valid());
    assert!(time2.is_valid());

    Ok(())
}
