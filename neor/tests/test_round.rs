use neor::{r, Converter, Result};

#[tokio::test]
async fn test_round_data() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data_obtained: u8 = r.round(12.345).run(&conn).await?.unwrap().parse()?;
    let data_obtained2: u8 = r.expr(12.345).round().run(&conn).await?.unwrap().parse()?;
    let data_obtained3: u8 = r.round(r.expr(12.345)).run(&conn).await?.unwrap().parse()?;

    assert!(
        data_obtained == 12 && data_obtained == data_obtained2 && data_obtained == data_obtained3
    );

    Ok(())
}
