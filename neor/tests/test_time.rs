use neor::types::Time;
use neor::{args, r, Converter, Result};
use time::macros::{date, offset, time};

#[tokio::test]
async fn test_time_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let date = date!(1986 - 11 - 3);
    let timezone = offset!(+01:00);
    let time = time!(09:30:40);

    let date_time = r.time(args!(date, time, timezone));
    let time1 = date_time.value();
    let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(time2 == time1);

    Ok(())
}
