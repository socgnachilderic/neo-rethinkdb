use neor::{r, Converter, Result};

#[tokio::test]
async fn test_floor_data() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data_obtained: i8 = r.floor(-12.345).run(&conn).await?.unwrap().parse()?;
    let data_obtained2: i8 = r.expr(-12.345).floor().run(&conn).await?.unwrap().parse()?;
    let data_obtained3: i8 = r
        .floor(r.expr(-12.345))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(
        data_obtained == -13 && data_obtained == data_obtained2 && data_obtained == data_obtained3
    );

    Ok(())
}
