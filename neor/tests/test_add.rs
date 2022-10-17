use neor::types::Time;
use neor::{r, Converter, Result};

#[tokio::test]
async fn test_add_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response1: u8 = (r.expr(2) + 2).run(&conn).await?.unwrap().parse()?;
    let response2: String = (r.expr("foo") + "bar" + "baz")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;
    let response3: Vec<String> = (r.expr(["foo", "bar"]) + ["buzz"])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;
    let response4: Time = (r.now().cmd() + 365 * 24 * 60 * 60)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert_eq!(response1, 4);
    assert_eq!(response2, "foobarbaz");
    assert_eq!(response3, ["foo", "bar", "buzz"]);
    assert_ne!(response4.epoch_time, 0.);

    Ok(())
}
