use neor::types::Time;
use neor::{r, Converter, Result};

#[tokio::test]
async fn test_now_time() -> Result<()> {
    let conn = r.connection().connect().await?;
    let time1 = r.now().value();
    let time2: Time = r.now().cmd().run(&conn).await?.unwrap().parse()?;

    assert!(time1.is_valid());
    assert!(time2.is_valid());

    Ok(())
}
