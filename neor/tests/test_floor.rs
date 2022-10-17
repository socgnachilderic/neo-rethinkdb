use neor::{r, Converter, Result};

#[tokio::test]
async fn test_floor_data() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: i8 = r.floor(-12.345).run(&conn).await?.unwrap().parse()?;
    let data_obtained2: i8 = r.expr(-12.345).floor().run(&conn).await?.unwrap().parse()?;
    let data_obtained3: i8 = r
        .floor(r.expr(-12.345))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == -13 && response == data_obtained2 && response == data_obtained3);

    Ok(())
}
