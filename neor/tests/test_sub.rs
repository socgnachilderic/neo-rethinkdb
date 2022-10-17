use neor::types::Time;
use neor::{r, Converter, Result};

#[tokio::test]
async fn test_sub_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response1: u8 = (r.expr(2) - 2).run(&conn).await?.unwrap().parse()?;
    let response2: Time = (r.now().cmd() - 365 * 24 * 60 * 60)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert_eq!(response1, 0);
    assert!(!response2.epoch_time.is_nan());

    Ok(())
}
