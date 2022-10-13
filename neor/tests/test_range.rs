use neor::{args, r, Converter, Result};

#[tokio::test]
async fn test_range_data() -> Result<()> {
    let data = [0, 1, 2, 3];
    let data2 = [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];
    let conn = r.connection().connect().await?;
    let response: [isize; 4] = r.range(4).run(&conn).await?.unwrap().parse()?;
    let response2: [isize; 4] = r.range(()).limit(4).run(&conn).await?.unwrap().parse()?;
    let response3: [isize; 11] = r.range(args!(-5, 6)).run(&conn).await?.unwrap().parse()?;

    assert!(response == data);
    assert!(response2 == data);
    assert!(response3 == data2);

    Ok(())
}
