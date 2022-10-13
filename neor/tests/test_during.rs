use neor::{args, r, Converter, Result};
use time::macros::{date, offset};

#[tokio::test]
async fn test_during_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let start_date = r.time(args!(date!(2022 - 08 - 01), offset!(UTC)));
    let end_date = r.time(args!(date!(2022 - 12 - 31), offset!(UTC)));

    let datetime = r.epoch_time(1661990400)?;

    let response = datetime
        .clone()
        .during(start_date.clone(), end_date.clone(), None);
    let response2: bool = response.clone().cmd().run(&conn).await?.unwrap().parse()?;
    let response3: bool = datetime
        .cmd()
        .during(args!(start_date, end_date))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response2 == response3 && response2 == response.value());

    Ok(())
}
