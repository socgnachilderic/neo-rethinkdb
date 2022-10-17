use neor::arguments::RandomOption;
use neor::{args, r, Converter, Result};

#[tokio::test]
async fn test_random_data() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data_obtained1: f64 = r.random(()).run(&conn).await?.unwrap().parse()?;
    let data_obtained2: isize = r.random(100.).run(&conn).await?.unwrap().parse()?;
    let data_obtained3: f64 = r
        .random(args!(-100.52, -10.71, RandomOption::default().float(true)))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained1.is_normal());
    assert!(data_obtained2 >= 0);
    assert!(data_obtained3.is_normal());

    Ok(())
}
