use neor::types::Time;
use neor::{args, r, Converter, Result};
use time::macros::offset;

#[tokio::test]
async fn test_iso8601_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let date_time = r.iso8601("1986-11-03T08:30:00-07:00")?;
    let time1 = date_time.value();
    let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(time2 == time1);

    Ok(())
}

#[tokio::test]
async fn test_iso8601_ops_with_default_timezone() -> Result<()> {
    let conn = r.connection().connect().await?;

    let date_time = r.iso8601(args!("1986-11-03T08:30:00", offset!(+01:00)))?;
    let time1 = date_time.value();
    let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(time2 == time1);

    Ok(())
}
