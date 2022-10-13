use neor::{r, Converter, Result};

#[tokio::test]
async fn test_day_of_year_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let day_of_year = r.now().day_of_year();
    let day_of_year1 = day_of_year.clone().value();
    let day_of_year2: u16 = day_of_year.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(day_of_year1 == day_of_year2);

    Ok(())
}
